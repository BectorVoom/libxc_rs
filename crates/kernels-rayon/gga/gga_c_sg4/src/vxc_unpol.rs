//! GGA_C_SG4 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_sg4.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_sg4_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t10 = t4 * t6 / t7;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t24 = t20 * t5 / t21;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.0621814) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t58 = t34 * t34;
            let t59 = ((t33).select(t58, f64x8::splat(1.0)));
            let t60 = ((v_sigma).sqrt());
            let t61 = t60 * v_sigma;
            let t62 = v_rho * v_rho;
            let t63 = t62 * t62;
            let t64 = f64x8::splat(1.0) / t63;
            let t66 = t59 * t59;
            let t67 = t66 * t59;
            let t68 = f64x8::splat(1.0) / t67;
            let t70 = f64x8::splat(1.0) / t13 / t10;
            let t71 = t68 * t70;
            let t74 = (simd::pow(t59, f64x8::splat(0.05) * t61 * t64 * t71));
            let t75 = (simd::ln(f64x8::splat(2.0)));
            let t76 = f64x8::splat(1.0) - t75;
            let t77 = t74 * t76;
            let t78 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = t79 * t67;
            let t82 = f64x8::splat(1.0) / t7 / v_rho;
            let t84 = t39 * t39;
            let t86 = f64x8::splat(1.0) / t59;
            let t87 = f64x8::splat(1.0) / t13;
            let t88 = t86 * t87;
            let t90 = (simd::exp(-t24 / f64x8::splat(4.0)));
            let t91 = f64x8::splat(1.0) - t90;
            let t92 = t88 * t91;
            let t95 = f64x8::splat(0.07963845034287749) + f64x8::splat(0.0175) * t60 * t82 * t84 * t92;
            let t97 = f64x8::splat(1.0) / t7 / t62;
            let t100 = f64x8::splat(1.0) / t66;
            let t102 = f64x8::splat(1.0) / t3;
            let t104 = t100 * t18 * t102 * t5;
            let t107 = f64x8::splat(1.0) / t76;
            let t108 = t95 * t107;
            let t113 = (simd::exp(-(-t32 + t57) * t107 * t78 * t68));
            let t114 = t113 - f64x8::splat(1.0);
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t78 * t115;
            let t117 = v_sigma * v_sigma;
            let t118 = t116 * t117;
            let t119 = t108 * t118;
            let t121 = f64x8::splat(1.0) / t21 / t63;
            let t122 = t121 * t84;
            let t123 = t66 * t66;
            let t124 = f64x8::splat(1.0) / t123;
            let t126 = f64x8::splat(1.0) / t19;
            let t127 = t1 * t126;
            let t128 = t127 * t6;
            let t129 = t122 * t124 * t128;
            let t132 = v_sigma * t97 * t39 * t104 / f64x8::splat(96.0) + t119 * t129 / f64x8::splat(3072.0);
            let t133 = t95 * t132;
            let t134 = t107 * t78;
            let t135 = t116 * t132;
            let t137 = t108 * t135 + f64x8::splat(1.0);
            let t138 = f64x8::splat(1.0) / t137;
            let t139 = t134 * t138;
            let t141 = t133 * t139 + f64x8::splat(1.0);
            let t142 = (simd::ln(t141));
            let t144 = t77 * t80 * t142;
            let tzk0 = -t32 + t57 + t144;
            acc_zk = tzk0;
            let t145 = t6 * t82;
            let t147 = t4 * t145 * t30;
            let t148 = f64x8::splat(0.0011073470983333333) * t147;
            let t149 = t26 * t26;
            let t150 = f64x8::splat(1.0) / t149;
            let t151 = t12 * t150;
            let t152 = t87 * t1;
            let t153 = t3 * t6;
            let t154 = t153 * t82;
            let t155 = t152 * t154;
            let t157 = t4 * t145;
            let t159 = ((t10).sqrt());
            let t160 = t159 * t1;
            let t161 = t160 * t154;
            let t165 = t5 / t21 / v_rho;
            let t166 = t20 * t165;
            let t168 = -f64x8::splat(0.632975) * t155 - f64x8::splat(0.29896666666666666) * t157 - f64x8::splat(0.1023875) * t161 - f64x8::splat(0.08215666666666667) * t166;
            let t169 = f64x8::splat(1.0) / t29;
            let t170 = t168 * t169;
            let t171 = t151 * t170;
            let t172 = f64x8::splat(1.0) * t171;
            let t173 = t43 * t1;
            let t176 = t173 * t153 * t82 * t54;
            let t177 = f64x8::splat(0.00018311447306006544) * t176;
            let t178 = t43 * t45;
            let t179 = t50 * t50;
            let t180 = f64x8::splat(1.0) / t179;
            let t185 = -f64x8::splat(0.8630833333333333) * t155 - f64x8::splat(0.301925) * t157 - f64x8::splat(0.05501625) * t161 - f64x8::splat(0.082785) * t166;
            let t187 = f64x8::splat(1.0) / t53;
            let t188 = t180 * t185 * t187;
            let t189 = t178 * t188;
            let t190 = f64x8::splat(0.5848223622634646) * t189;
            let t191 = t63 * v_rho;
            let t192 = f64x8::splat(1.0) / t191;
            let t197 = f64x8::splat(1.0) / t7 / t191;
            let t202 = f64x8::splat(1.0) / t13 / t24 / f64x8::splat(4.0);
            let t203 = t202 * t1;
            let t204 = t203 * t153;
            let t207 = -f64x8::splat(0.2) * t61 * t192 * t71 + f64x8::splat(0.025) * t61 * t197 * t68 * t204;
            let t208 = t74 * t207;
            let t209 = (simd::ln(t59));
            let t211 = t76 * t79;
            let t213 = t211 * t67 * t142;
            let t214 = t208 * t209 * t213;
            let t215 = t77 * t79;
            let t221 = f64x8::splat(1.0) / t21 / t62;
            let t223 = t84 * t86;
            let t226 = t4 * t6;
            let t227 = t70 * t91 * t226;
            let t230 = t62 * v_rho;
            let t231 = f64x8::splat(1.0) / t230;
            let t235 = t19 * t5;
            let t236 = t235 * t90;
            let t237 = t87 * t18 * t236;
            let t240 = -f64x8::splat(0.023333333333333334) * t60 * t97 * t84 * t92 + f64x8::splat(0.002916666666666667) * t60 * t221 * t223 * t227 - f64x8::splat(0.002916666666666667) * t60 * t231 * t223 * t237;
            let t241 = t240 * t132;
            let t244 = f64x8::splat(1.0) / t7 / t230;
            let t249 = t240 * t107;
            let t250 = t249 * t118;
            let t253 = t76 * t76;
            let t254 = f64x8::splat(1.0) / t253;
            let t255 = t95 * t254;
            let t256 = t78 * t78;
            let t257 = t255 * t256;
            let t258 = t114 * t114;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t259 * t117;
            let t261 = t260 * t121;
            let t262 = t257 * t261;
            let t264 = f64x8::splat(1.0) / t123 / t67;
            let t266 = t84 * t264 * t1;
            let t267 = t126 * t6;
            let t268 = t148 + t172 - t177 - t190;
            let t269 = t268 * t113;
            let t271 = t266 * t267 * t269;
            let t275 = f64x8::splat(1.0) / t21 / t191;
            let t276 = t275 * t84;
            let t278 = t276 * t124 * t128;
            let t281 = -f64x8::splat(7.0) / f64x8::splat(288.0) * v_sigma * t244 * t39 * t104 + t250 * t129 / f64x8::splat(3072.0) + t262 * t271 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t119 * t278;
            let t282 = t95 * t281;
            let t284 = t133 * t107;
            let t285 = t137 * t137;
            let t286 = f64x8::splat(1.0) / t285;
            let t287 = t78 * t286;
            let t289 = t256 * t259;
            let t290 = t255 * t289;
            let t291 = t132 * t268;
            let t292 = t68 * t113;
            let t293 = t291 * t292;
            let t295 = t116 * t281;
            let t297 = t108 * t295 + t249 * t135 + t290 * t293;
            let t298 = t287 * t297;
            let t300 = t241 * t139 + t282 * t139 - t284 * t298;
            let t302 = f64x8::splat(1.0) / t141;
            let t303 = t67 * t300 * t302;
            let t304 = t215 * t303;
            let tvrho0 = -t32 + t57 + t144 + v_rho * (t148 + t172 - t177 - t190 + t214 + t304);
            acc_vrho = tvrho0;
            let t307 = t74 * t60;
            let t310 = t76 * t142;
            let t311 = t70 * t209 * t310;
            let t313 = f64x8::splat(0.007599088773175333) * t307 * t64 * t311;
            let t314 = f64x8::splat(1.0) / t60;
            let t315 = t314 * t82;
            let t316 = t315 * t223;
            let t317 = t87 * t91;
            let t318 = t132 * t107;
            let t319 = t318 * t138;
            let t320 = t317 * t319;
            let t325 = t18 * t102;
            let t326 = t325 * t5;
            let t329 = t63 * t62;
            let t330 = f64x8::splat(1.0) / t329;
            let t331 = t61 * t330;
            let t332 = t123 * t59;
            let t333 = f64x8::splat(1.0) / t332;
            let t334 = t39 * t333;
            let t335 = t334 * t87;
            let t337 = t91 * t107;
            let t338 = t337 * t115;
            let t339 = t338 * t128;
            let t342 = t116 * v_sigma;
            let t343 = t108 * t342;
            let t346 = t97 * t39 * t100 * t326 / f64x8::splat(96.0) + f64x8::splat(5.622333236297649e-05) * t331 * t335 * t339 + t343 * t129 / f64x8::splat(1536.0);
            let t347 = t95 * t346;
            let t349 = t107 * t115;
            let t350 = t349 * t132;
            let t351 = t317 * t350;
            let t354 = t116 * t346;
            let t356 = f64x8::splat(0.08635903850953189) * t316 * t351 + t108 * t354;
            let t357 = t287 * t356;
            let t359 = f64x8::splat(0.08635903850953189) * t316 * t320 + t347 * t139 - t284 * t357;
            let t360 = t67 * t359;
            let t361 = t360 * t302;
            let t362 = t215 * t361;
            let tvsigma0 = v_rho * (t313 + t362);
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

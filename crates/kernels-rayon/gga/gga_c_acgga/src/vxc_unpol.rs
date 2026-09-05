//! GGA_C_ACGGA vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_acgga.c`
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

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_acgga_vxc_unpol(
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
            let t58 = (simd::ln(f64x8::splat(2.0)));
            let t59 = f64x8::splat(1.0) - t58;
            let t60 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t59 * t61;
            let t63 = t34 * t34;
            let t64 = ((t33).select(t63, f64x8::splat(1.0)));
            let t65 = t64 * t64;
            let t66 = t65 * t64;
            let t67 = v_rho * v_rho;
            let t69 = f64x8::splat(1.0) / t7 / t67;
            let t70 = v_sigma * t69;
            let t71 = f64x8::splat(1.0) / t65;
            let t72 = t39 * t71;
            let t73 = t70 * t72;
            let t74 = f64x8::splat(1.0) / t3;
            let t75 = t18 * t74;
            let t76 = ((v_sigma).sqrt());
            let t78 = f64x8::splat(1.0) / t7 / v_rho;
            let t80 = t39 * t39;
            let t81 = f64x8::splat(1.0) / t64;
            let t82 = t80 * t81;
            let t83 = f64x8::splat(1.0) / t13;
            let t84 = t82 * t83;
            let t85 = t76 * t78 * t84;
            let t87 = f64x8::splat(4.5) + t85 / f64x8::splat(4.0);
            let t88 = t5 * t87;
            let t90 = f64x8::splat(4.5) + f64x8::splat(0.36675) * t85;
            let t91 = f64x8::splat(1.0) / t90;
            let t93 = t75 * t88 * t91;
            let t96 = f64x8::splat(1.0) / t59;
            let t99 = f64x8::splat(1.0) / t66;
            let t100 = t60 * t99;
            let t102 = (simd::exp(-(-t32 + t57) * t96 * t100));
            let t103 = t102 - f64x8::splat(1.0);
            let t104 = f64x8::splat(1.0) / t103;
            let t105 = t96 * t104;
            let t106 = v_sigma * v_sigma;
            let t107 = t67 * t67;
            let t109 = f64x8::splat(1.0) / t21 / t107;
            let t110 = t106 * t109;
            let t112 = t105 * t110 * t80;
            let t113 = t65 * t65;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t114 * t1;
            let t116 = f64x8::splat(1.0) / t19;
            let t117 = t115 * t116;
            let t118 = t87 * t87;
            let t119 = t6 * t118;
            let t120 = t90 * t90;
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t119 * t121;
            let t123 = t117 * t122;
            let t126 = t73 * t93 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t112 * t123;
            let t127 = t126 * t96;
            let t130 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t105 * t126;
            let t131 = f64x8::splat(1.0) / t130;
            let t134 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t127 * t131;
            let t135 = (simd::ln(t134));
            let t137 = t62 * t66 * t135;
            let tzk0 = -t32 + t57 + t137;
            acc_zk = tzk0;
            let t138 = t6 * t78;
            let t140 = t4 * t138 * t30;
            let t141 = f64x8::splat(0.0011073470983333333) * t140;
            let t142 = t26 * t26;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t12 * t143;
            let t145 = t83 * t1;
            let t146 = t3 * t6;
            let t147 = t146 * t78;
            let t148 = t145 * t147;
            let t150 = t4 * t138;
            let t152 = ((t10).sqrt());
            let t153 = t152 * t1;
            let t154 = t153 * t147;
            let t159 = t20 * t5 / t21 / v_rho;
            let t161 = -f64x8::splat(0.632975) * t148 - f64x8::splat(0.29896666666666666) * t150 - f64x8::splat(0.1023875) * t154 - f64x8::splat(0.08215666666666667) * t159;
            let t162 = f64x8::splat(1.0) / t29;
            let t163 = t161 * t162;
            let t164 = t144 * t163;
            let t165 = f64x8::splat(1.0) * t164;
            let t166 = t43 * t1;
            let t169 = t166 * t146 * t78 * t54;
            let t170 = f64x8::splat(0.00018311447306006544) * t169;
            let t171 = t43 * t45;
            let t172 = t50 * t50;
            let t173 = f64x8::splat(1.0) / t172;
            let t178 = -f64x8::splat(0.8630833333333333) * t148 - f64x8::splat(0.301925) * t150 - f64x8::splat(0.05501625) * t154 - f64x8::splat(0.082785) * t159;
            let t180 = f64x8::splat(1.0) / t53;
            let t181 = t173 * t178 * t180;
            let t182 = t171 * t181;
            let t183 = f64x8::splat(0.5848223622634646) * t182;
            let t184 = t67 * v_rho;
            let t186 = f64x8::splat(1.0) / t7 / t184;
            let t187 = v_sigma * t186;
            let t188 = t187 * t72;
            let t192 = t76 * t69 * t84;
            let t195 = f64x8::splat(1.0) / t21 / t67;
            let t199 = f64x8::splat(1.0) / t13 / t10;
            let t201 = t199 * t1 * t146;
            let t202 = t76 * t195 * t82 * t201;
            let t204 = -t192 / f64x8::splat(3.0) + t202 / f64x8::splat(24.0);
            let t205 = t5 * t204;
            let t207 = t75 * t205 * t91;
            let t210 = t72 * t18;
            let t211 = t70 * t210;
            let t212 = t74 * t5;
            let t213 = t87 * t121;
            let t216 = -f64x8::splat(0.489) * t192 + f64x8::splat(0.061125) * t202;
            let t218 = t212 * t213 * t216;
            let t221 = t59 * t59;
            let t222 = f64x8::splat(1.0) / t221;
            let t223 = t103 * t103;
            let t224 = f64x8::splat(1.0) / t223;
            let t225 = t222 * t224;
            let t226 = t225 * t106;
            let t227 = t109 * t80;
            let t229 = f64x8::splat(1.0) / t113 / t66;
            let t230 = t229 * t1;
            let t231 = t227 * t230;
            let t232 = t226 * t231;
            let t233 = t116 * t6;
            let t234 = t233 * t118;
            let t235 = t141 + t165 - t170 - t183;
            let t236 = t121 * t235;
            let t237 = t60 * t102;
            let t238 = t236 * t237;
            let t239 = t234 * t238;
            let t242 = t107 * v_rho;
            let t244 = f64x8::splat(1.0) / t21 / t242;
            let t245 = t106 * t244;
            let t247 = t105 * t245 * t80;
            let t250 = t105 * t106;
            let t251 = t227 * t114;
            let t252 = t250 * t251;
            let t253 = t1 * t116;
            let t254 = t253 * t6;
            let t256 = t254 * t213 * t204;
            let t260 = f64x8::splat(1.0) / t120 / t90;
            let t261 = t118 * t260;
            let t263 = t254 * t261 * t216;
            let t266 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t188 * t93 + t73 * t207 / f64x8::splat(96.0) - t211 * t218 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t232 * t239 - f64x8::splat(0.0010003937560882938) * t247 * t123 + f64x8::splat(0.0004287401811806974) * t252 * t256 - f64x8::splat(0.0004287401811806974) * t252 * t263;
            let t267 = t266 * t96;
            let t270 = t130 * t130;
            let t271 = f64x8::splat(1.0) / t270;
            let t272 = t225 * t126;
            let t273 = t235 * t60;
            let t274 = t99 * t102;
            let t275 = t273 * t274;
            let t280 = f64x8::splat(0.6585449182935511) * t272 * t275 + f64x8::splat(0.6585449182935511) * t105 * t266;
            let t281 = t271 * t280;
            let t284 = f64x8::splat(0.6585449182935511) * t267 * t131 - f64x8::splat(0.6585449182935511) * t127 * t281;
            let t286 = f64x8::splat(1.0) / t134;
            let t288 = t62 * t66 * t284 * t286;
            let tvrho0 = -t32 + t57 + t137 + v_rho * (t141 + t165 - t170 - t183 + t288);
            acc_vrho = tvrho0;
            let t291 = v_rho * t59;
            let t292 = t291 * t61;
            let t293 = t69 * t39;
            let t294 = t71 * t18;
            let t295 = t293 * t294;
            let t297 = t212 * t87 * t91;
            let t301 = f64x8::splat(1.0) / t21 / t184;
            let t302 = t76 * t301;
            let t303 = t99 * t18;
            let t304 = t302 * t303;
            let t305 = t83 * t91;
            let t306 = t212 * t305;
            let t309 = t213 * t83;
            let t310 = t212 * t309;
            let t313 = v_sigma * t109;
            let t315 = t105 * t313 * t80;
            let t318 = t76 * v_sigma;
            let t319 = t105 * t318;
            let t320 = t107 * t67;
            let t321 = f64x8::splat(1.0) / t320;
            let t322 = t321 * t39;
            let t323 = t113 * t64;
            let t324 = f64x8::splat(1.0) / t323;
            let t325 = t322 * t324;
            let t326 = t319 * t325;
            let t327 = t254 * t309;
            let t331 = t254 * t261 * t83;
            let t334 = t295 * t297 / f64x8::splat(96.0) + t304 * t306 / f64x8::splat(384.0) - f64x8::splat(0.0038203125) * t304 * t310 + f64x8::splat(0.0004287401811806974) * t315 * t123 + f64x8::splat(0.00010718504529517435) * t326 * t327 - f64x8::splat(0.00015724046144802075) * t326 * t331;
            let t335 = t334 * t96;
            let t338 = t126 * t222;
            let t339 = t271 * t104;
            let t340 = t339 * t334;
            let t343 = f64x8::splat(0.6585449182935511) * t335 * t131 - f64x8::splat(0.43368140941025995) * t338 * t340;
            let t344 = t66 * t343;
            let t345 = t344 * t286;
            let tvsigma0 = t292 * t345;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}

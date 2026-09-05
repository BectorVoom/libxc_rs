//! GGA_C_PBELOC vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbeloc.c`
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
pub fn gga_c_pbeloc_vxc_unpol(
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
            let t74 = f64x8::splat(1.0) / t3;
            let t75 = t18 * t74;
            let t77 = (simd::exp(-t24 / f64x8::splat(4.0)));
            let t78 = f64x8::splat(1.0) - t77;
            let t79 = t5 * t78;
            let t80 = t75 * t79;
            let t83 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t70 * t72 * t80;
            let t85 = t71 * t18;
            let t87 = t85 * t74 * t5;
            let t90 = f64x8::splat(1.0) / t59;
            let t91 = t83 * t90;
            let t94 = f64x8::splat(1.0) / t66;
            let t97 = (simd::exp(-(-t32 + t57) * t90 * t60 * t94));
            let t98 = t97 - f64x8::splat(1.0);
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t60 * t99;
            let t101 = v_sigma * v_sigma;
            let t102 = t100 * t101;
            let t103 = t91 * t102;
            let t104 = t67 * t67;
            let t106 = f64x8::splat(1.0) / t21 / t104;
            let t107 = t39 * t39;
            let t108 = t106 * t107;
            let t109 = t65 * t65;
            let t110 = f64x8::splat(1.0) / t109;
            let t112 = f64x8::splat(1.0) / t19;
            let t114 = t1 * t112 * t6;
            let t115 = t108 * t110 * t114;
            let t118 = t70 * t39 * t87 / f64x8::splat(96.0) + t103 * t115 / f64x8::splat(3072.0);
            let t119 = t83 * t118;
            let t120 = t90 * t60;
            let t121 = t100 * t118;
            let t123 = t121 * t91 + f64x8::splat(1.0);
            let t124 = f64x8::splat(1.0) / t123;
            let t125 = t120 * t124;
            let t127 = t119 * t125 + f64x8::splat(1.0);
            let t128 = (simd::ln(t127));
            let t130 = t62 * t66 * t128;
            let tzk0 = -t32 + t57 + t130;
            acc_zk = tzk0;
            let t132 = f64x8::splat(1.0) / t7 / v_rho;
            let t133 = t6 * t132;
            let t135 = t4 * t133 * t30;
            let t136 = f64x8::splat(0.0011073470983333333) * t135;
            let t137 = t26 * t26;
            let t138 = f64x8::splat(1.0) / t137;
            let t139 = t12 * t138;
            let t141 = f64x8::splat(1.0) / t13 * t1;
            let t142 = t3 * t6;
            let t143 = t142 * t132;
            let t144 = t141 * t143;
            let t146 = t4 * t133;
            let t148 = ((t10).sqrt());
            let t149 = t148 * t1;
            let t150 = t149 * t143;
            let t155 = t20 * t5 / t21 / v_rho;
            let t157 = -f64x8::splat(0.632975) * t144 - f64x8::splat(0.29896666666666666) * t146 - f64x8::splat(0.1023875) * t150 - f64x8::splat(0.08215666666666667) * t155;
            let t158 = f64x8::splat(1.0) / t29;
            let t159 = t157 * t158;
            let t160 = t139 * t159;
            let t161 = f64x8::splat(1.0) * t160;
            let t162 = t43 * t1;
            let t165 = t162 * t142 * t132 * t54;
            let t166 = f64x8::splat(0.00018311447306006544) * t165;
            let t167 = t43 * t45;
            let t168 = t50 * t50;
            let t169 = f64x8::splat(1.0) / t168;
            let t174 = -f64x8::splat(0.8630833333333333) * t144 - f64x8::splat(0.301925) * t146 - f64x8::splat(0.05501625) * t150 - f64x8::splat(0.082785) * t155;
            let t176 = f64x8::splat(1.0) / t53;
            let t177 = t169 * t174 * t176;
            let t178 = t167 * t177;
            let t179 = f64x8::splat(0.5848223622634646) * t178;
            let t180 = t67 * v_rho;
            let t182 = f64x8::splat(1.0) / t7 / t180;
            let t183 = v_sigma * t182;
            let t187 = f64x8::splat(1.0) / t104;
            let t190 = t6 * t77;
            let t191 = t4 * t190;
            let t194 = -f64x8::splat(0.0019444444444444444) * t183 * t72 * t80 - f64x8::splat(0.0004166666666666667) * v_sigma * t187 * t72 * t191;
            let t195 = t194 * t118;
            let t200 = t194 * t90;
            let t201 = t200 * t102;
            let t204 = t59 * t59;
            let t205 = f64x8::splat(1.0) / t204;
            let t206 = t83 * t205;
            let t207 = t60 * t60;
            let t208 = t206 * t207;
            let t209 = t98 * t98;
            let t210 = f64x8::splat(1.0) / t209;
            let t211 = t210 * t101;
            let t212 = t211 * t106;
            let t213 = t208 * t212;
            let t214 = t109 * t66;
            let t215 = f64x8::splat(1.0) / t214;
            let t217 = t107 * t215 * t1;
            let t218 = t112 * t6;
            let t219 = t136 + t161 - t166 - t179;
            let t220 = t219 * t97;
            let t222 = t217 * t218 * t220;
            let t225 = t104 * v_rho;
            let t227 = f64x8::splat(1.0) / t21 / t225;
            let t228 = t227 * t107;
            let t230 = t228 * t110 * t114;
            let t233 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t183 * t39 * t87 + t201 * t115 / f64x8::splat(3072.0) + t213 * t222 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t103 * t230;
            let t234 = t83 * t233;
            let t236 = t119 * t90;
            let t237 = t123 * t123;
            let t238 = f64x8::splat(1.0) / t237;
            let t239 = t60 * t238;
            let t241 = t207 * t210;
            let t242 = t206 * t241;
            let t243 = t118 * t219;
            let t244 = t94 * t97;
            let t245 = t243 * t244;
            let t247 = t100 * t233;
            let t249 = t121 * t200 + t242 * t245 + t247 * t91;
            let t250 = t239 * t249;
            let t252 = t125 * t195 + t125 * t234 - t236 * t250;
            let t254 = f64x8::splat(1.0) / t127;
            let t256 = t62 * t66 * t252 * t254;
            let tvrho0 = -t32 + t57 + t130 + v_rho * (t136 + t161 - t166 - t179 + t256);
            acc_vrho = tvrho0;
            let t259 = v_rho * t59;
            let t260 = t259 * t61;
            let t261 = t69 * t39;
            let t262 = t85 * t74;
            let t263 = t261 * t262;
            let t265 = t118 * t90 * t124;
            let t266 = t79 * t265;
            let t270 = t75 * t5;
            let t271 = t261 * t71 * t270;
            let t273 = t104 * t180;
            let t274 = f64x8::splat(1.0) / t273;
            let t276 = f64x8::splat(1.0) / t109 / t65;
            let t278 = t274 * t276 * t78;
            let t279 = t90 * t99;
            let t280 = t279 * t101;
            let t283 = t100 * v_sigma;
            let t284 = t91 * t283;
            let t287 = t271 / f64x8::splat(96.0) + f64x8::splat(0.00020186378047070194) * t278 * t280 + t284 * t115 / f64x8::splat(1536.0);
            let t288 = t83 * t287;
            let t290 = t279 * t118;
            let t291 = t79 * t290;
            let t294 = t100 * t287;
            let t296 = f64x8::splat(0.008224670334241133) * t263 * t291 + t91 * t294;
            let t297 = t239 * t296;
            let t299 = f64x8::splat(0.008224670334241133) * t263 * t266 + t288 * t125 - t236 * t297;
            let t300 = t66 * t299;
            let t301 = t300 * t254;
            let tvsigma0 = t260 * t301;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}

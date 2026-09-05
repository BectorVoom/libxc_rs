//! GGA_C_GAPLOC exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_gaploc.c`
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
pub fn gga_c_gaploc_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
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
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t4 * t9;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t22 = f64x8::splat(1.0) / t21;
            let t24 = t20 * t5 * t22;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.062182) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t35 = t34 * zeta_threshold;
            let t36 = ((t33).select(t35, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t42 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) * t42;
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t50;
            let t54 = (simd::ln(t53));
            let t55 = t45 * t54;
            let t57 = f64x8::splat(0.019751789702565206) * t43 * t55;
            let t58 = t3 * t2;
            let t59 = f64x8::splat(1.0) / t58;
            let t60 = t18 * t59;
            let t61 = t7 * v_rho;
            let t62 = t6 * t61;
            let t63 = f64x8::splat(1.0) / v_rho;
            let t66 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t13 + f64x8::splat(0.0123825) * t10;
            let t69 = f64x8::splat(1.0) + t13 * t66 / f64x8::splat(2.0);
            let t70 = t69 * t69;
            let t71 = f64x8::splat(1.0) / t70;
            let t74 = -t32 + t57;
            let t76 = t1 * t58;
            let t77 = f64x8::splat(1.0) / t61;
            let t78 = t6 * t77;
            let t79 = t76 * t78;
            let t82 = t18 * t19 * t2;
            let t84 = f64x8::splat(1.0) / t21 / v_rho;
            let t85 = t5 * t84;
            let t86 = t82 * t85;
            let t88 = v_rho * v_rho;
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t94 = t1 * t3 / t91;
            let t96 = f64x8::splat(1.0) / t7 / t88;
            let t97 = t6 * t96;
            let t98 = t94 * t97;
            let t100 = -f64x8::splat(0.005977859662531589) * t63 + f64x8::splat(0.001317375) * t79 - f64x8::splat(0.00023775) * t86 + f64x8::splat(6.474423634745383e-06) * t89 - f64x8::splat(5.40140625e-07) * t98;
            let t102 = f64x8::splat(0.0011713266981940448) * t63 * t71 - t74 * t100;
            let t104 = t60 * t62 * t102;
            let t105 = f64x8::splat(1.0) / t19;
            let t106 = t1 * t105;
            let t107 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t108 = t106 * t107;
            let t109 = t13 * t10;
            let t110 = t21 * t109;
            let t111 = f64x8::splat(1.0) / t69;
            let t115 = t74 * t74;
            let t117 = f64x8::splat(0.0019711289) * t108 * t110 * t111 - f64x8::splat(2.0) * t115;
            let t118 = f64x8::splat(1.0) / t117;
            let t119 = (simd::cbrt(f64x8::splat(9.0)));
            let t120 = t119 * t119;
            let t121 = t118 * t120;
            let t122 = f64x8::splat(M_CBRT6);
            let t123 = t122 * t122;
            let t124 = (simd::cbrt(t91));
            let t127 = ((v_sigma).sqrt());
            let t131 = t123 / t124 * t127 * t77 * t39 / f64x8::splat(12.0);
            let t132 = t127 * v_sigma;
            let t133 = t88 * v_rho;
            let t134 = ((v_rho).sqrt());
            let t136 = f64x8::splat(1.0) / t134 / t133;
            let t137 = t132 * t136;
            let t138 = t34 * t34;
            let t139 = ((t33).select(t138, f64x8::splat(1.0)));
            let t140 = t139 * t139;
            let t142 = f64x8::splat(1.0) / t140 / t139;
            let t143 = ((f64x8::splat(3.0)).sqrt());
            let t145 = ((t2).sqrt());
            let t146 = f64x8::splat(1.0) / t145;
            let t148 = t137 * t142 * t143 * t146;
            let t150 = f64x8::splat(8.54613) + t148 / f64x8::splat(64.0);
            let t152 = f64x8::splat(1.0) + t148 / f64x8::splat(192.0);
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t150 * t153;
            let t155 = (simd::pow(t131, t154));
            let t156 = t124 * t124;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = t122 * t157;
            let t160 = f64x8::splat(1.0) / t21 / t88;
            let t162 = t39 * t39;
            let t166 = f64x8::splat(14.709046) + t158 * v_sigma * t160 * t162 / f64x8::splat(24.0);
            let t168 = f64x8::splat(1.0) + t155;
            let t169 = f64x8::splat(1.0) / t168;
            let t171 = t121 * t155 * t166 * t169;
            let t174 = -t32 + t57 + f64x8::splat(0.02845500663567615) * t104 * t171;
            let t175 = ((f64x8::splat(4.0)).sqrt());
            let t176 = t74 * t175;
            let t177 = t109 * t111;
            let t180 = t6 * t21;
            let t184 = f64x8::splat(0.00619125) * t176 * t177 - f64x8::splat(0.07959333333333334) * t106 * t180 * t100;
            let t185 = t184 * t118;
            let t186 = t120 * t155;
            let t187 = t185 * t186;
            let t188 = t21 * t166;
            let t189 = t188 * t169;
            let t190 = t106 * t189;
            let t193 = t102 * t118;
            let t194 = t155 * t155;
            let t195 = t119 * t194;
            let t196 = t193 * t195;
            let t197 = t166 * t166;
            let t198 = t61 * t197;
            let t199 = t168 * t168;
            let t200 = f64x8::splat(1.0) / t199;
            let t201 = t198 * t200;
            let t202 = t60 * t201;
            let t205 = f64x8::splat(1.0) + f64x8::splat(0.3575048995185043) * t187 * t190 - f64x8::splat(1.1502877786176224) * t196 * t202;
            let t206 = f64x8::splat(1.0) / t205;
            let t207 = t174 * t206;
            let t208 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t210 = ((t208).select(t35, f64x8::splat(2.0) * t39));
            let t211 = (f64x8::splat(0.0)).simd_le(zeta_threshold);
            let t212 = ((t211).select(t35, f64x8::splat(0.0)));
            let t214 = (t210 + t212 - f64x8::splat(2.0)) * t42;
            let t216 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t10;
            let t221 = f64x8::splat(7.05945) * t13 + f64x8::splat(1.549425) * t10 + f64x8::splat(0.420775) * t16 + f64x8::splat(0.1562925) * t24;
            let t224 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t221;
            let t225 = (simd::ln(t224));
            let t230 = t214 * (-f64x8::splat(0.03109) * t216 * t225 + t32 - f64x8::splat(0.019751789702565206) * t55);
            let t232 = f64x8::splat(0.019751789702565206) * t214 * t55;
            let t235 = f64x8::splat(1.49676) + f64x8::splat(0.00089527) * t13 + f64x8::splat(0.011799625) * t10;
            let t238 = f64x8::splat(1.0) + t13 * t235 / f64x8::splat(2.0);
            let t239 = t238 * t238;
            let t240 = f64x8::splat(1.0) / t239;
            let t243 = -t32 + t230 + t232;
            let t249 = -f64x8::splat(0.0077371026992393175) * t63 + f64x8::splat(0.00187495875) * t79 - f64x8::splat(0.000362780625) * t86 + f64x8::splat(1.0208501871552144e-05) * t89 - f64x8::splat(8.659659375e-07) * t98;
            let t251 = f64x8::splat(0.0010636476373080148) * t63 * t240 - t243 * t249;
            let t253 = t60 * t62 * t251;
            let t254 = f64x8::splat(1.0) / t238;
            let t258 = t243 * t243;
            let t260 = f64x8::splat(0.0005076591995833333) * t108 * t110 * t254 - f64x8::splat(2.0) * t258;
            let t261 = f64x8::splat(1.0) / t260;
            let t262 = t261 * t120;
            let t263 = ((t208).select(t138, t162));
            let t264 = ((t211).select(t138, f64x8::splat(0.0)));
            let t266 = t263 / f64x8::splat(2.0) + t264 / f64x8::splat(2.0);
            let t267 = t266 * t266;
            let t269 = f64x8::splat(1.0) / t267 / t266;
            let t272 = t137 * t269 * t143 * t146;
            let t274 = f64x8::splat(8.54613) + t272 / f64x8::splat(64.0);
            let t276 = f64x8::splat(1.0) + t272 / f64x8::splat(192.0);
            let t277 = f64x8::splat(1.0) / t276;
            let t278 = t274 * t277;
            let t279 = (simd::pow(t131, t278));
            let t281 = f64x8::splat(1.0) + t279;
            let t282 = f64x8::splat(1.0) / t281;
            let t284 = t262 * t279 * t166 * t282;
            let t287 = -t32 + t230 + t232 + f64x8::splat(0.007690526230142224) * t253 * t284;
            let t288 = t243 * t175;
            let t289 = t109 * t254;
            let t295 = f64x8::splat(0.0058998125) * t288 * t289 - f64x8::splat(0.021511666666666665) * t106 * t180 * t249;
            let t296 = t295 * t261;
            let t297 = t120 * t279;
            let t298 = t296 * t297;
            let t299 = t188 * t282;
            let t300 = t106 * t299;
            let t303 = t251 * t261;
            let t304 = t279 * t279;
            let t305 = t119 * t304;
            let t306 = t303 * t305;
            let t307 = t281 * t281;
            let t308 = f64x8::splat(1.0) / t307;
            let t309 = t198 * t308;
            let t310 = t60 * t309;
            let t313 = f64x8::splat(1.0) + f64x8::splat(0.3575048995185043) * t298 * t300 - f64x8::splat(1.1502877786176224) * t306 * t310;
            let t314 = f64x8::splat(1.0) / t313;
            let t317 = t43 * (t287 * t314 - t207);
            let tzk0 = t207 + t317;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

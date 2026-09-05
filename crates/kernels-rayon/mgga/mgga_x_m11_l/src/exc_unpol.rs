//! MGGA_X_M11_L exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_m11_l.c`
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
pub fn mgga_x_m11_l_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_a_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_b_9: f64,
    param_b_10: f64,
    param_b_11: f64,
    param_b_0: f64,
    param_c_1: f64,
    param_c_2: f64,
    param_c_3: f64,
    param_c_4: f64,
    param_c_5: f64,
    param_c_6: f64,
    param_c_7: f64,
    param_c_8: f64,
    param_c_9: f64,
    param_c_10: f64,
    param_c_11: f64,
    param_c_0: f64,
    param_d_1: f64,
    param_d_2: f64,
    param_d_3: f64,
    param_d_4: f64,
    param_d_5: f64,
    param_d_6: f64,
    param_d_7: f64,
    param_d_8: f64,
    param_d_9: f64,
    param_d_10: f64,
    param_d_11: f64,
    param_d_0: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a_1 = f64x8::splat(param_a_1);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_a_3 = f64x8::splat(param_a_3);
    let param_a_4 = f64x8::splat(param_a_4);
    let param_a_5 = f64x8::splat(param_a_5);
    let param_a_6 = f64x8::splat(param_a_6);
    let param_a_7 = f64x8::splat(param_a_7);
    let param_a_8 = f64x8::splat(param_a_8);
    let param_a_9 = f64x8::splat(param_a_9);
    let param_a_10 = f64x8::splat(param_a_10);
    let param_a_11 = f64x8::splat(param_a_11);
    let param_a_0 = f64x8::splat(param_a_0);
    let param_b_1 = f64x8::splat(param_b_1);
    let param_b_2 = f64x8::splat(param_b_2);
    let param_b_3 = f64x8::splat(param_b_3);
    let param_b_4 = f64x8::splat(param_b_4);
    let param_b_5 = f64x8::splat(param_b_5);
    let param_b_6 = f64x8::splat(param_b_6);
    let param_b_7 = f64x8::splat(param_b_7);
    let param_b_8 = f64x8::splat(param_b_8);
    let param_b_9 = f64x8::splat(param_b_9);
    let param_b_10 = f64x8::splat(param_b_10);
    let param_b_11 = f64x8::splat(param_b_11);
    let param_b_0 = f64x8::splat(param_b_0);
    let param_c_1 = f64x8::splat(param_c_1);
    let param_c_2 = f64x8::splat(param_c_2);
    let param_c_3 = f64x8::splat(param_c_3);
    let param_c_4 = f64x8::splat(param_c_4);
    let param_c_5 = f64x8::splat(param_c_5);
    let param_c_6 = f64x8::splat(param_c_6);
    let param_c_7 = f64x8::splat(param_c_7);
    let param_c_8 = f64x8::splat(param_c_8);
    let param_c_9 = f64x8::splat(param_c_9);
    let param_c_10 = f64x8::splat(param_c_10);
    let param_c_11 = f64x8::splat(param_c_11);
    let param_c_0 = f64x8::splat(param_c_0);
    let param_d_1 = f64x8::splat(param_d_1);
    let param_d_2 = f64x8::splat(param_d_2);
    let param_d_3 = f64x8::splat(param_d_3);
    let param_d_4 = f64x8::splat(param_d_4);
    let param_d_5 = f64x8::splat(param_d_5);
    let param_d_6 = f64x8::splat(param_d_6);
    let param_d_7 = f64x8::splat(param_d_7);
    let param_d_8 = f64x8::splat(param_d_8);
    let param_d_9 = f64x8::splat(param_d_9);
    let param_d_10 = f64x8::splat(param_d_10);
    let param_d_11 = f64x8::splat(param_d_11);
    let param_d_0 = f64x8::splat(param_d_0);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t13 = (t12).simd_le(zeta_threshold);
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = ((t13).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = (simd::cbrt(f64x8::splat(9.0)));
            let t22 = t21 * t21;
            let t24 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t25 = t24 * t24;
            let t27 = t22 * t25 * param_hyb_omega_0;
            let t30 = ((t13).select(t14, t16));
            let t31 = f64x8::splat(1.0) / t30;
            let t34 = t27 * t4 / t19 * t31 / f64x8::splat(18.0);
            let t35 = (f64x8::splat(1.35)).simd_le(t34);
            let t36 = (f64x8::splat(1.35)).simd_lt(t34);
            let t37 = ((t36).select(t34, f64x8::splat(1.35)));
            let t38 = t37 * t37;
            let t41 = t38 * t38;
            let t42 = f64x8::splat(1.0) / t41;
            let t44 = t41 * t38;
            let t45 = f64x8::splat(1.0) / t44;
            let t47 = t41 * t41;
            let t48 = f64x8::splat(1.0) / t47;
            let t51 = f64x8::splat(1.0) / t47 / t38;
            let t54 = f64x8::splat(1.0) / t47 / t41;
            let t57 = f64x8::splat(1.0) / t47 / t44;
            let t59 = t47 * t47;
            let t60 = f64x8::splat(1.0) / t59;
            let t63 = ((t36).select(f64x8::splat(1.35), t34));
            let t64 = ((f64x8::splat(M_PI)).sqrt());
            let t65 = f64x8::splat(1.0) / t63;
            let t67 = (simd::erf(t65 / f64x8::splat(2.0)));
            let t69 = t63 * t63;
            let t70 = f64x8::splat(1.0) / t69;
            let t72 = (simd::exp(-t70 / f64x8::splat(4.0)));
            let t73 = t72 - f64x8::splat(1.0);
            let t76 = t72 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t69 * t73;
            let t79 = f64x8::splat(2.0) * t63 * t76 + t64 * t67;
            let t83 = ((t35).select(f64x8::splat(1.0) / t38 / f64x8::splat(36.0) - t42 / f64x8::splat(960.0) + t45 / f64x8::splat(26880.0) - t48 / f64x8::splat(829440.0) + t51 / f64x8::splat(28385280.0) - t54 / f64x8::splat(1073479680.0) + t57 / f64x8::splat(44590694400.0) - t60 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t63 * t79));
            let t84 = f64x8::splat(M_CBRT6);
            let t85 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t86 = (simd::cbrt(t85));
            let t87 = t86 * t86;
            let t88 = f64x8::splat(1.0) / t87;
            let t89 = t84 * t88;
            let t90 = f64x8::splat(M_CBRT2);
            let t91 = t90 * t90;
            let t92 = v_sigma * t91;
            let t93 = v_rho * v_rho;
            let t94 = t19 * t19;
            let t96 = f64x8::splat(1.0) / t94 / t93;
            let t98 = t89 * t92 * t96;
            let t100 = f64x8::splat(0.804) + f64x8::splat(0.00914625) * t98;
            let t103 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t100;
            let t105 = param_a_1;
            let t106 = t84 * t84;
            let t108 = f64x8::splat(3.0) / f64x8::splat(10.0) * t106 * t87;
            let t109 = v_tau * t91;
            let t111 = f64x8::splat(1.0) / t94 / v_rho;
            let t112 = t109 * t111;
            let t113 = t108 - t112;
            let t114 = t105 * t113;
            let t115 = t108 + t112;
            let t116 = f64x8::splat(1.0) / t115;
            let t118 = param_a_2;
            let t119 = t113 * t113;
            let t120 = t118 * t119;
            let t121 = t115 * t115;
            let t122 = f64x8::splat(1.0) / t121;
            let t124 = param_a_3;
            let t125 = t119 * t113;
            let t126 = t124 * t125;
            let t127 = t121 * t115;
            let t128 = f64x8::splat(1.0) / t127;
            let t130 = param_a_4;
            let t131 = t119 * t119;
            let t132 = t130 * t131;
            let t133 = t121 * t121;
            let t134 = f64x8::splat(1.0) / t133;
            let t136 = param_a_5;
            let t137 = t131 * t113;
            let t138 = t136 * t137;
            let t139 = t133 * t115;
            let t140 = f64x8::splat(1.0) / t139;
            let t142 = param_a_6;
            let t143 = t131 * t119;
            let t144 = t142 * t143;
            let t145 = t133 * t121;
            let t146 = f64x8::splat(1.0) / t145;
            let t148 = param_a_7;
            let t149 = t131 * t125;
            let t150 = t148 * t149;
            let t151 = t133 * t127;
            let t152 = f64x8::splat(1.0) / t151;
            let t154 = param_a_8;
            let t155 = t131 * t131;
            let t156 = t154 * t155;
            let t157 = t133 * t133;
            let t158 = f64x8::splat(1.0) / t157;
            let t160 = param_a_9;
            let t161 = t155 * t113;
            let t162 = t160 * t161;
            let t164 = f64x8::splat(1.0) / t157 / t115;
            let t166 = param_a_10;
            let t167 = t155 * t119;
            let t168 = t166 * t167;
            let t170 = f64x8::splat(1.0) / t157 / t121;
            let t172 = param_a_11;
            let t173 = t155 * t125;
            let t174 = t172 * t173;
            let t176 = f64x8::splat(1.0) / t157 / t127;
            let t178 = t114 * t116 + t120 * t122 + t126 * t128 + t132 * t134 + t138 * t140 + t144 * t146 + t150 * t152 + t156 * t158 + t162 * t164 + t168 * t170 + t174 * t176 + param_a_0;
            let t181 = (simd::exp(-f64x8::splat(0.009318900220671557) * t98));
            let t183 = f64x8::splat(1.552) - f64x8::splat(0.552) * t181;
            let t185 = param_b_1;
            let t186 = t185 * t113;
            let t188 = param_b_2;
            let t189 = t188 * t119;
            let t191 = param_b_3;
            let t192 = t191 * t125;
            let t194 = param_b_4;
            let t195 = t194 * t131;
            let t197 = param_b_5;
            let t198 = t197 * t137;
            let t200 = param_b_6;
            let t201 = t200 * t143;
            let t203 = param_b_7;
            let t204 = t203 * t149;
            let t206 = param_b_8;
            let t207 = t206 * t155;
            let t209 = param_b_9;
            let t210 = t209 * t161;
            let t212 = param_b_10;
            let t213 = t212 * t167;
            let t215 = param_b_11;
            let t216 = t215 * t173;
            let t218 = t186 * t116 + t189 * t122 + t192 * t128 + t195 * t134 + t198 * t140 + t201 * t146 + t204 * t152 + t207 * t158 + t210 * t164 + t213 * t170 + t216 * t176 + param_b_0;
            let t220 = t103 * t178 + t183 * t218;
            let t222 = f64x8::splat(1.0) - t83;
            let t224 = param_c_1;
            let t225 = t224 * t113;
            let t227 = param_c_2;
            let t228 = t227 * t119;
            let t230 = param_c_3;
            let t231 = t230 * t125;
            let t233 = param_c_4;
            let t234 = t233 * t131;
            let t236 = param_c_5;
            let t237 = t236 * t137;
            let t239 = param_c_6;
            let t240 = t239 * t143;
            let t242 = param_c_7;
            let t243 = t242 * t149;
            let t245 = param_c_8;
            let t246 = t245 * t155;
            let t248 = param_c_9;
            let t249 = t248 * t161;
            let t251 = param_c_10;
            let t252 = t251 * t167;
            let t254 = param_c_11;
            let t255 = t254 * t173;
            let t257 = t225 * t116 + t228 * t122 + t231 * t128 + t234 * t134 + t237 * t140 + t240 * t146 + t243 * t152 + t246 * t158 + t249 * t164 + t252 * t170 + t255 * t176 + param_c_0;
            let t260 = param_d_1;
            let t261 = t260 * t113;
            let t263 = param_d_2;
            let t264 = t263 * t119;
            let t266 = param_d_3;
            let t267 = t266 * t125;
            let t269 = param_d_4;
            let t270 = t269 * t131;
            let t272 = param_d_5;
            let t273 = t272 * t137;
            let t275 = param_d_6;
            let t276 = t275 * t143;
            let t278 = param_d_7;
            let t279 = t278 * t149;
            let t281 = param_d_8;
            let t282 = t281 * t155;
            let t284 = param_d_9;
            let t285 = t284 * t161;
            let t287 = param_d_10;
            let t288 = t287 * t167;
            let t290 = param_d_11;
            let t291 = t290 * t173;
            let t293 = t261 * t116 + t264 * t122 + t267 * t128 + t270 * t134 + t273 * t140 + t276 * t146 + t279 * t152 + t282 * t158 + t285 * t164 + t288 * t170 + t291 * t176 + param_d_0;
            let t295 = t103 * t257 + t183 * t293;
            let t297 = t83 * t220 + t222 * t295;
            let t301 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t297));
            let tzk0 = f64x8::splat(2.0) * t301;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

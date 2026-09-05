//! MGGA_X_M11_L fxc unpol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_x_m11_l_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
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
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
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
            let t303 = t18 / t94;
            let t307 = t38 * t37;
            let t308 = f64x8::splat(1.0) / t307;
            let t314 = t27 * t4 / t19 / v_rho * t31 / f64x8::splat(54.0);
            let t315 = ((t36).select(-t314, f64x8::splat(0.0)));
            let t318 = t41 * t37;
            let t319 = f64x8::splat(1.0) / t318;
            let t322 = t41 * t307;
            let t323 = f64x8::splat(1.0) / t322;
            let t327 = f64x8::splat(1.0) / t47 / t37;
            let t331 = f64x8::splat(1.0) / t47 / t307;
            let t335 = f64x8::splat(1.0) / t47 / t318;
            let t339 = f64x8::splat(1.0) / t47 / t322;
            let t343 = f64x8::splat(1.0) / t59 / t37;
            let t347 = ((t36).select(f64x8::splat(0.0), -t314));
            let t349 = t72 * t70;
            let t353 = t69 * t63;
            let t354 = f64x8::splat(1.0) / t353;
            let t358 = t63 * t73;
            let t363 = t354 * t347 * t72 / f64x8::splat(2.0) - f64x8::splat(4.0) * t358 * t347 - t65 * t347 * t72;
            let t366 = -t349 * t347 + f64x8::splat(2.0) * t347 * t76 + f64x8::splat(2.0) * t63 * t363;
            let t370 = ((t35).select(-t308 * t315 / f64x8::splat(18.0) + t319 * t315 / f64x8::splat(240.0) - t323 * t315 / f64x8::splat(4480.0) + t327 * t315 / f64x8::splat(103680.0) - t331 * t315 / f64x8::splat(2838528.0) + t335 * t315 / f64x8::splat(89456640.0) - t339 * t315 / f64x8::splat(3185049600.0) + t343 * t315 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t347 * t79 - f64x8::splat(8.0) / f64x8::splat(3.0) * t63 * t366));
            let t372 = t100 * t100;
            let t375 = f64x8::splat(1.0) / t372 * t84 * t88;
            let t376 = t93 * v_rho;
            let t378 = f64x8::splat(1.0) / t94 / t376;
            let t383 = t105 * v_tau;
            let t384 = t91 * t96;
            let t385 = t384 * t116;
            let t388 = t114 * t122;
            let t389 = t109 * t96;
            let t392 = t118 * t113;
            let t393 = t392 * t122;
            let t396 = t120 * t128;
            let t399 = t124 * t119;
            let t400 = t399 * t128;
            let t403 = t126 * t134;
            let t406 = t130 * t125;
            let t407 = t406 * t134;
            let t410 = t132 * t140;
            let t413 = t136 * t131;
            let t414 = t413 * t140;
            let t417 = t138 * t146;
            let t420 = t142 * t137;
            let t421 = t420 * t146;
            let t424 = f64x8::splat(5.0) / f64x8::splat(3.0) * t383 * t385 + f64x8::splat(5.0) / f64x8::splat(3.0) * t388 * t389 + f64x8::splat(10.0) / f64x8::splat(3.0) * t393 * t389 + f64x8::splat(10.0) / f64x8::splat(3.0) * t396 * t389 + f64x8::splat(5.0) * t400 * t389 + f64x8::splat(5.0) * t403 * t389 + f64x8::splat(20.0) / f64x8::splat(3.0) * t407 * t389 + f64x8::splat(20.0) / f64x8::splat(3.0) * t410 * t389 + f64x8::splat(25.0) / f64x8::splat(3.0) * t414 * t389 + f64x8::splat(25.0) / f64x8::splat(3.0) * t417 * t389 + f64x8::splat(10.0) * t421 * t389;
            let t425 = t144 * t152;
            let t428 = t148 * t143;
            let t429 = t428 * t152;
            let t432 = t150 * t158;
            let t435 = t154 * t149;
            let t436 = t435 * t158;
            let t439 = t156 * t164;
            let t442 = t160 * t155;
            let t443 = t442 * t164;
            let t446 = t162 * t170;
            let t449 = t166 * t161;
            let t450 = t449 * t170;
            let t453 = t168 * t176;
            let t456 = t172 * t167;
            let t457 = t456 * t176;
            let t461 = f64x8::splat(1.0) / t157 / t133;
            let t462 = t174 * t461;
            let t465 = f64x8::splat(10.0) * t425 * t389 + f64x8::splat(35.0) / f64x8::splat(3.0) * t429 * t389 + f64x8::splat(35.0) / f64x8::splat(3.0) * t432 * t389 + f64x8::splat(40.0) / f64x8::splat(3.0) * t436 * t389 + f64x8::splat(40.0) / f64x8::splat(3.0) * t439 * t389 + f64x8::splat(15.0) * t443 * t389 + f64x8::splat(15.0) * t446 * t389 + f64x8::splat(50.0) / f64x8::splat(3.0) * t450 * t389 + f64x8::splat(50.0) / f64x8::splat(3.0) * t453 * t389 + f64x8::splat(55.0) / f64x8::splat(3.0) * t457 * t389 + f64x8::splat(55.0) / f64x8::splat(3.0) * t462 * t389;
            let t466 = t424 + t465;
            let t468 = t89 * v_sigma;
            let t469 = t91 * t378;
            let t470 = t181 * t218;
            let t474 = t185 * v_tau;
            let t477 = t186 * t122;
            let t480 = t188 * t113;
            let t481 = t480 * t122;
            let t484 = t189 * t128;
            let t487 = t191 * t119;
            let t488 = t487 * t128;
            let t491 = t192 * t134;
            let t494 = t194 * t125;
            let t495 = t494 * t134;
            let t498 = t195 * t140;
            let t501 = t197 * t131;
            let t502 = t501 * t140;
            let t505 = t198 * t146;
            let t508 = t200 * t137;
            let t509 = t508 * t146;
            let t512 = f64x8::splat(5.0) / f64x8::splat(3.0) * t474 * t385 + f64x8::splat(5.0) / f64x8::splat(3.0) * t477 * t389 + f64x8::splat(10.0) / f64x8::splat(3.0) * t481 * t389 + f64x8::splat(10.0) / f64x8::splat(3.0) * t484 * t389 + f64x8::splat(5.0) * t488 * t389 + f64x8::splat(5.0) * t491 * t389 + f64x8::splat(20.0) / f64x8::splat(3.0) * t495 * t389 + f64x8::splat(20.0) / f64x8::splat(3.0) * t498 * t389 + f64x8::splat(25.0) / f64x8::splat(3.0) * t502 * t389 + f64x8::splat(25.0) / f64x8::splat(3.0) * t505 * t389 + f64x8::splat(10.0) * t509 * t389;
            let t513 = t201 * t152;
            let t516 = t203 * t143;
            let t517 = t516 * t152;
            let t520 = t204 * t158;
            let t523 = t206 * t149;
            let t524 = t523 * t158;
            let t527 = t207 * t164;
            let t530 = t209 * t155;
            let t531 = t530 * t164;
            let t534 = t210 * t170;
            let t537 = t212 * t161;
            let t538 = t537 * t170;
            let t541 = t213 * t176;
            let t544 = t215 * t167;
            let t545 = t544 * t176;
            let t548 = t216 * t461;
            let t551 = f64x8::splat(10.0) * t513 * t389 + f64x8::splat(35.0) / f64x8::splat(3.0) * t517 * t389 + f64x8::splat(35.0) / f64x8::splat(3.0) * t520 * t389 + f64x8::splat(40.0) / f64x8::splat(3.0) * t524 * t389 + f64x8::splat(40.0) / f64x8::splat(3.0) * t527 * t389 + f64x8::splat(15.0) * t531 * t389 + f64x8::splat(15.0) * t534 * t389 + f64x8::splat(50.0) / f64x8::splat(3.0) * t538 * t389 + f64x8::splat(50.0) / f64x8::splat(3.0) * t541 * t389 + f64x8::splat(55.0) / f64x8::splat(3.0) * t545 * t389 + f64x8::splat(55.0) / f64x8::splat(3.0) * t548 * t389;
            let t552 = t512 + t551;
            let t554 = -f64x8::splat(0.01576608624) * t375 * t92 * t378 * t178 + t103 * t466 - f64x8::splat(0.013717421124828532) * t468 * t469 * t470 + t183 * t552;
            let t561 = t224 * v_tau;
            let t564 = t225 * t122;
            let t567 = t227 * t113;
            let t568 = t567 * t122;
            let t571 = t228 * t128;
            let t574 = t230 * t119;
            let t575 = t574 * t128;
            let t578 = t231 * t134;
            let t581 = t233 * t125;
            let t582 = t581 * t134;
            let t585 = t234 * t140;
            let t588 = t236 * t131;
            let t589 = t588 * t140;
            let t592 = t237 * t146;
            let t595 = t239 * t137;
            let t596 = t595 * t146;
            let t599 = f64x8::splat(5.0) / f64x8::splat(3.0) * t561 * t385 + f64x8::splat(5.0) / f64x8::splat(3.0) * t564 * t389 + f64x8::splat(10.0) / f64x8::splat(3.0) * t568 * t389 + f64x8::splat(10.0) / f64x8::splat(3.0) * t571 * t389 + f64x8::splat(5.0) * t575 * t389 + f64x8::splat(5.0) * t578 * t389 + f64x8::splat(20.0) / f64x8::splat(3.0) * t582 * t389 + f64x8::splat(20.0) / f64x8::splat(3.0) * t585 * t389 + f64x8::splat(25.0) / f64x8::splat(3.0) * t589 * t389 + f64x8::splat(25.0) / f64x8::splat(3.0) * t592 * t389 + f64x8::splat(10.0) * t596 * t389;
            let t600 = t240 * t152;
            let t603 = t242 * t143;
            let t604 = t603 * t152;
            let t607 = t243 * t158;
            let t610 = t245 * t149;
            let t611 = t610 * t158;
            let t614 = t246 * t164;
            let t617 = t248 * t155;
            let t618 = t617 * t164;
            let t621 = t249 * t170;
            let t624 = t251 * t161;
            let t625 = t624 * t170;
            let t628 = t252 * t176;
            let t631 = t254 * t167;
            let t632 = t631 * t176;
            let t635 = t255 * t461;
            let t638 = f64x8::splat(10.0) * t600 * t389 + f64x8::splat(35.0) / f64x8::splat(3.0) * t604 * t389 + f64x8::splat(35.0) / f64x8::splat(3.0) * t607 * t389 + f64x8::splat(40.0) / f64x8::splat(3.0) * t611 * t389 + f64x8::splat(40.0) / f64x8::splat(3.0) * t614 * t389 + f64x8::splat(15.0) * t618 * t389 + f64x8::splat(15.0) * t621 * t389 + f64x8::splat(50.0) / f64x8::splat(3.0) * t625 * t389 + f64x8::splat(50.0) / f64x8::splat(3.0) * t628 * t389 + f64x8::splat(55.0) / f64x8::splat(3.0) * t632 * t389 + f64x8::splat(55.0) / f64x8::splat(3.0) * t635 * t389;
            let t639 = t599 + t638;
            let t641 = t181 * t293;
            let t645 = t260 * v_tau;
            let t648 = t261 * t122;
            let t651 = t263 * t113;
            let t652 = t651 * t122;
            let t655 = t264 * t128;
            let t658 = t266 * t119;
            let t659 = t658 * t128;
            let t662 = t267 * t134;
            let t665 = t269 * t125;
            let t666 = t665 * t134;
            let t669 = t270 * t140;
            let t672 = t272 * t131;
            let t673 = t672 * t140;
            let t676 = t273 * t146;
            let t679 = t275 * t137;
            let t680 = t679 * t146;
            let t683 = f64x8::splat(5.0) / f64x8::splat(3.0) * t645 * t385 + f64x8::splat(5.0) / f64x8::splat(3.0) * t648 * t389 + f64x8::splat(10.0) / f64x8::splat(3.0) * t652 * t389 + f64x8::splat(10.0) / f64x8::splat(3.0) * t655 * t389 + f64x8::splat(5.0) * t659 * t389 + f64x8::splat(5.0) * t662 * t389 + f64x8::splat(20.0) / f64x8::splat(3.0) * t666 * t389 + f64x8::splat(20.0) / f64x8::splat(3.0) * t669 * t389 + f64x8::splat(25.0) / f64x8::splat(3.0) * t673 * t389 + f64x8::splat(25.0) / f64x8::splat(3.0) * t676 * t389 + f64x8::splat(10.0) * t680 * t389;
            let t684 = t276 * t152;
            let t687 = t278 * t143;
            let t688 = t687 * t152;
            let t691 = t279 * t158;
            let t694 = t281 * t149;
            let t695 = t694 * t158;
            let t698 = t282 * t164;
            let t701 = t284 * t155;
            let t702 = t701 * t164;
            let t705 = t285 * t170;
            let t708 = t287 * t161;
            let t709 = t708 * t170;
            let t712 = t288 * t176;
            let t715 = t290 * t167;
            let t716 = t715 * t176;
            let t719 = t291 * t461;
            let t722 = f64x8::splat(10.0) * t684 * t389 + f64x8::splat(35.0) / f64x8::splat(3.0) * t688 * t389 + f64x8::splat(35.0) / f64x8::splat(3.0) * t691 * t389 + f64x8::splat(40.0) / f64x8::splat(3.0) * t695 * t389 + f64x8::splat(40.0) / f64x8::splat(3.0) * t698 * t389 + f64x8::splat(15.0) * t702 * t389 + f64x8::splat(15.0) * t705 * t389 + f64x8::splat(50.0) / f64x8::splat(3.0) * t709 * t389 + f64x8::splat(50.0) / f64x8::splat(3.0) * t712 * t389 + f64x8::splat(55.0) / f64x8::splat(3.0) * t716 * t389 + f64x8::splat(55.0) / f64x8::splat(3.0) * t719 * t389;
            let t723 = t683 + t722;
            let t725 = -f64x8::splat(0.01576608624) * t375 * t92 * t378 * t257 + t103 * t639 - f64x8::splat(0.013717421124828532) * t468 * t469 * t641 + t183 * t723;
            let t727 = t370 * t220 + t222 * t725 - t370 * t295 + t83 * t554;
            let t732 = ((t3).select(f64x8::splat(0.0), -t7 * t303 * t297 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t727));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t732 + f64x8::splat(2.0) * t301;
            acc_vrho = tvrho0;
            let t738 = t89 * t91;
            let t739 = t96 * t181;
            let t743 = f64x8::splat(0.00591228234) * t375 * t384 * t178 + f64x8::splat(0.0051440329218107) * t738 * t739 * t218;
            let t751 = f64x8::splat(0.00591228234) * t375 * t384 * t257 + f64x8::splat(0.0051440329218107) * t738 * t739 * t293;
            let t753 = t222 * t751 + t83 * t743;
            let t757 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t753));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t757;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t759 = t105 * t91;
            let t760 = t111 * t116;
            let t762 = t122 * t91;
            let t763 = t762 * t111;
            let t767 = t128 * t91;
            let t768 = t767 * t111;
            let t773 = t134 * t91;
            let t774 = t773 * t111;
            let t779 = t140 * t91;
            let t780 = t779 * t111;
            let t785 = t146 * t91;
            let t786 = t785 * t111;
            let t791 = -t114 * t763 - f64x8::splat(2.0) * t120 * t768 - f64x8::splat(3.0) * t126 * t774 - f64x8::splat(4.0) * t132 * t780 - f64x8::splat(5.0) * t138 * t786 - f64x8::splat(2.0) * t392 * t763 - f64x8::splat(3.0) * t399 * t768 - f64x8::splat(4.0) * t406 * t774 - f64x8::splat(5.0) * t413 * t780 - f64x8::splat(6.0) * t420 * t786 - t759 * t760;
            let t792 = t152 * t91;
            let t793 = t792 * t111;
            let t798 = t158 * t91;
            let t799 = t798 * t111;
            let t804 = t164 * t91;
            let t805 = t804 * t111;
            let t810 = t170 * t91;
            let t811 = t810 * t111;
            let t816 = t176 * t91;
            let t817 = t816 * t111;
            let t822 = t461 * t91;
            let t823 = t822 * t111;
            let t826 = -f64x8::splat(6.0) * t144 * t793 - f64x8::splat(7.0) * t150 * t799 - f64x8::splat(8.0) * t156 * t805 - f64x8::splat(9.0) * t162 * t811 - f64x8::splat(10.0) * t168 * t817 - f64x8::splat(11.0) * t174 * t823 - f64x8::splat(7.0) * t428 * t793 - f64x8::splat(8.0) * t435 * t799 - f64x8::splat(9.0) * t442 * t805 - f64x8::splat(10.0) * t449 * t811 - f64x8::splat(11.0) * t456 * t817;
            let t827 = t791 + t826;
            let t829 = t185 * t91;
            let t850 = -t186 * t763 - f64x8::splat(2.0) * t189 * t768 - f64x8::splat(3.0) * t192 * t774 - f64x8::splat(4.0) * t195 * t780 - f64x8::splat(5.0) * t198 * t786 - f64x8::splat(2.0) * t480 * t763 - f64x8::splat(3.0) * t487 * t768 - f64x8::splat(4.0) * t494 * t774 - f64x8::splat(5.0) * t501 * t780 - f64x8::splat(6.0) * t508 * t786 - t829 * t760;
            let t873 = -f64x8::splat(6.0) * t201 * t793 - f64x8::splat(7.0) * t204 * t799 - f64x8::splat(8.0) * t207 * t805 - f64x8::splat(9.0) * t210 * t811 - f64x8::splat(10.0) * t213 * t817 - f64x8::splat(11.0) * t216 * t823 - f64x8::splat(7.0) * t516 * t793 - f64x8::splat(8.0) * t523 * t799 - f64x8::splat(9.0) * t530 * t805 - f64x8::splat(10.0) * t537 * t811 - f64x8::splat(11.0) * t544 * t817;
            let t874 = t850 + t873;
            let t876 = t103 * t827 + t183 * t874;
            let t878 = t224 * t91;
            let t899 = -t225 * t763 - f64x8::splat(2.0) * t228 * t768 - f64x8::splat(3.0) * t231 * t774 - f64x8::splat(4.0) * t234 * t780 - f64x8::splat(5.0) * t237 * t786 - f64x8::splat(2.0) * t567 * t763 - f64x8::splat(3.0) * t574 * t768 - f64x8::splat(4.0) * t581 * t774 - f64x8::splat(5.0) * t588 * t780 - f64x8::splat(6.0) * t595 * t786 - t878 * t760;
            let t922 = -f64x8::splat(6.0) * t240 * t793 - f64x8::splat(7.0) * t243 * t799 - f64x8::splat(8.0) * t246 * t805 - f64x8::splat(9.0) * t249 * t811 - f64x8::splat(10.0) * t252 * t817 - f64x8::splat(11.0) * t255 * t823 - f64x8::splat(7.0) * t603 * t793 - f64x8::splat(8.0) * t610 * t799 - f64x8::splat(9.0) * t617 * t805 - f64x8::splat(10.0) * t624 * t811 - f64x8::splat(11.0) * t631 * t817;
            let t923 = t899 + t922;
            let t925 = t260 * t91;
            let t946 = -t261 * t763 - f64x8::splat(2.0) * t264 * t768 - f64x8::splat(3.0) * t267 * t774 - f64x8::splat(4.0) * t270 * t780 - f64x8::splat(5.0) * t273 * t786 - f64x8::splat(2.0) * t651 * t763 - f64x8::splat(3.0) * t658 * t768 - f64x8::splat(4.0) * t665 * t774 - f64x8::splat(5.0) * t672 * t780 - f64x8::splat(6.0) * t679 * t786 - t925 * t760;
            let t969 = -f64x8::splat(6.0) * t276 * t793 - f64x8::splat(7.0) * t279 * t799 - f64x8::splat(8.0) * t282 * t805 - f64x8::splat(9.0) * t285 * t811 - f64x8::splat(10.0) * t288 * t817 - f64x8::splat(11.0) * t291 * t823 - f64x8::splat(7.0) * t687 * t793 - f64x8::splat(8.0) * t694 * t799 - f64x8::splat(9.0) * t701 * t805 - f64x8::splat(10.0) * t708 * t811 - f64x8::splat(11.0) * t715 * t817;
            let t970 = t946 + t969;
            let t972 = t103 * t923 + t183 * t970;
            let t974 = t222 * t972 + t83 * t876;
            let t978 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t974));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t978;
            acc_vtau = tvtau0;
            let t981 = t18 * t111;
            let t988 = t315 * t315;
            let t996 = f64x8::splat(2.0) / f64x8::splat(81.0) * t27 * t4 / t19 / t93 * t31;
            let t997 = ((t36).select(t996, f64x8::splat(0.0)));
            let t1025 = f64x8::splat(1.0) / t59 / t38;
            let t1030 = t42 * t988 / f64x8::splat(6.0) - t308 * t997 / f64x8::splat(18.0) - t45 * t988 / f64x8::splat(48.0) + t319 * t997 / f64x8::splat(240.0) + t48 * t988 / f64x8::splat(640.0) - t323 * t997 / f64x8::splat(4480.0) - t51 * t988 / f64x8::splat(11520.0) + t327 * t997 / f64x8::splat(103680.0) + t54 * t988 / f64x8::splat(258048.0) - t331 * t997 / f64x8::splat(2838528.0) - t57 * t988 / f64x8::splat(6881280.0) + t335 * t997 / f64x8::splat(89456640.0) + t60 * t988 / f64x8::splat(212336640.0) - t339 * t997 / f64x8::splat(3185049600.0) - t1025 * t988 / f64x8::splat(7431782400.0) + t343 * t997 / f64x8::splat(126340300800.0);
            let t1031 = ((t36).select(f64x8::splat(0.0), t996));
            let t1036 = t69 * t69;
            let t1038 = f64x8::splat(1.0) / t1036 / t63;
            let t1039 = t347 * t347;
            let t1040 = t1038 * t1039;
            let t1043 = t72 * t354;
            let t1051 = f64x8::splat(1.0) / t1036;
            let t1059 = f64x8::splat(1.0) / t1036 / t69;
            let t1060 = t1059 * t1039;
            let t1071 = -f64x8::splat(2.0) * t1051 * t1039 * t72 + t354 * t1031 * t72 / f64x8::splat(2.0) + t1060 * t72 / f64x8::splat(4.0) - f64x8::splat(4.0) * t1039 * t73 - t70 * t1039 * t72 - f64x8::splat(4.0) * t358 * t1031 - t65 * t1031 * t72;
            let t1074 = -t1040 * t72 / f64x8::splat(2.0) + f64x8::splat(2.0) * t1043 * t1039 - t349 * t1031 + f64x8::splat(2.0) * t1031 * t76 + f64x8::splat(4.0) * t347 * t363 + f64x8::splat(2.0) * t63 * t1071;
            let t1078 = ((t35).select(t1030, -f64x8::splat(8.0) / f64x8::splat(3.0) * t1031 * t79 - f64x8::splat(16.0) / f64x8::splat(3.0) * t347 * t366 - f64x8::splat(8.0) / f64x8::splat(3.0) * t63 * t1074));
            let t1086 = f64x8::splat(1.0) / t86 / t85;
            let t1087 = f64x8::splat(1.0) / t372 / t100 * t106 * t1086;
            let t1088 = v_sigma * v_sigma;
            let t1089 = t1088 * t90;
            let t1090 = t93 * t93;
            let t1091 = t1090 * t376;
            let t1093 = f64x8::splat(1.0) / t19 / t1091;
            let t1099 = f64x8::splat(1.0) / t94 / t1090;
            let t1108 = t109 * t378;
            let t1135 = -f64x8::splat(40.0) * t446 * t1108 - f64x8::splat(400.0) / f64x8::splat(9.0) * t450 * t1108 - f64x8::splat(400.0) / f64x8::splat(9.0) * t453 * t1108 - f64x8::splat(440.0) / f64x8::splat(9.0) * t457 * t1108 - f64x8::splat(440.0) / f64x8::splat(9.0) * t462 * t1108 - f64x8::splat(200.0) / f64x8::splat(9.0) * t414 * t1108 - f64x8::splat(200.0) / f64x8::splat(9.0) * t417 * t1108 - f64x8::splat(80.0) / f64x8::splat(3.0) * t421 * t1108 - f64x8::splat(80.0) / f64x8::splat(3.0) * t425 * t1108 - f64x8::splat(280.0) / f64x8::splat(9.0) * t429 * t1108 - f64x8::splat(280.0) / f64x8::splat(9.0) * t432 * t1108 - f64x8::splat(320.0) / f64x8::splat(9.0) * t436 * t1108 - f64x8::splat(320.0) / f64x8::splat(9.0) * t439 * t1108;
            let t1152 = t172 * t161;
            let t1153 = t1152 * t176;
            let t1154 = v_tau * v_tau;
            let t1155 = t1154 * t90;
            let t1156 = t1090 * v_rho;
            let t1158 = f64x8::splat(1.0) / t19 / t1156;
            let t1159 = t1155 * t1158;
            let t1162 = t456 * t461;
            let t1166 = f64x8::splat(1.0) / t157 / t139;
            let t1167 = t174 * t1166;
            let t1170 = t148 * t137;
            let t1171 = t1170 * t152;
            let t1174 = t428 * t158;
            let t1177 = t150 * t164;
            let t1180 = -f64x8::splat(40.0) * t443 * t1108 - f64x8::splat(40.0) / f64x8::splat(9.0) * t388 * t1108 - f64x8::splat(80.0) / f64x8::splat(9.0) * t393 * t1108 - f64x8::splat(80.0) / f64x8::splat(9.0) * t396 * t1108 - f64x8::splat(40.0) / f64x8::splat(3.0) * t400 * t1108 - f64x8::splat(40.0) / f64x8::splat(3.0) * t403 * t1108 - f64x8::splat(160.0) / f64x8::splat(9.0) * t407 * t1108 - f64x8::splat(160.0) / f64x8::splat(9.0) * t410 * t1108 + f64x8::splat(5500.0) / f64x8::splat(9.0) * t1153 * t1159 + f64x8::splat(12100.0) / f64x8::splat(9.0) * t1162 * t1159 + f64x8::splat(2200.0) / f64x8::splat(3.0) * t1167 * t1159 + f64x8::splat(700.0) / f64x8::splat(3.0) * t1171 * t1159 + f64x8::splat(4900.0) / f64x8::splat(9.0) * t1174 * t1159 + f64x8::splat(2800.0) / f64x8::splat(9.0) * t1177 * t1159;
            let t1182 = t154 * t143;
            let t1183 = t1182 * t158;
            let t1186 = t435 * t164;
            let t1189 = t156 * t170;
            let t1192 = t160 * t149;
            let t1193 = t1192 * t164;
            let t1196 = t442 * t170;
            let t1199 = t162 * t176;
            let t1202 = t166 * t155;
            let t1203 = t1202 * t170;
            let t1206 = t449 * t176;
            let t1209 = t168 * t461;
            let t1212 = t124 * t113;
            let t1213 = t1212 * t128;
            let t1216 = t399 * t134;
            let t1219 = t126 * t140;
            let t1222 = t130 * t119;
            let t1223 = t1222 * t134;
            let t1226 = f64x8::splat(2800.0) / f64x8::splat(9.0) * t1183 * t1159 + f64x8::splat(6400.0) / f64x8::splat(9.0) * t1186 * t1159 + f64x8::splat(400.0) * t1189 * t1159 + f64x8::splat(400.0) * t1193 * t1159 + f64x8::splat(900.0) * t1196 * t1159 + f64x8::splat(500.0) * t1199 * t1159 + f64x8::splat(500.0) * t1203 * t1159 + f64x8::splat(10000.0) / f64x8::splat(9.0) * t1206 * t1159 + f64x8::splat(5500.0) / f64x8::splat(9.0) * t1209 * t1159 + f64x8::splat(100.0) / f64x8::splat(3.0) * t1213 * t1159 + f64x8::splat(100.0) * t1216 * t1159 + f64x8::splat(200.0) / f64x8::splat(3.0) * t1219 * t1159 + f64x8::splat(200.0) / f64x8::splat(3.0) * t1223 * t1159;
            let t1227 = t406 * t140;
            let t1230 = t132 * t146;
            let t1233 = t136 * t125;
            let t1234 = t1233 * t140;
            let t1237 = t413 * t146;
            let t1240 = t138 * t152;
            let t1243 = t142 * t131;
            let t1244 = t1243 * t146;
            let t1247 = t420 * t152;
            let t1250 = t144 * t158;
            let t1253 = t114 * t128;
            let t1256 = t392 * t128;
            let t1259 = t120 * t134;
            let t1262 = t105 * t1154;
            let t1263 = t90 * t1158;
            let t1264 = t1263 * t122;
            let t1267 = t118 * t1154;
            let t1270 = t469 * t116;
            let t1273 = f64x8::splat(1600.0) / f64x8::splat(9.0) * t1227 * t1159 + f64x8::splat(1000.0) / f64x8::splat(9.0) * t1230 * t1159 + f64x8::splat(1000.0) / f64x8::splat(9.0) * t1234 * t1159 + f64x8::splat(2500.0) / f64x8::splat(9.0) * t1237 * t1159 + f64x8::splat(500.0) / f64x8::splat(3.0) * t1240 * t1159 + f64x8::splat(500.0) / f64x8::splat(3.0) * t1244 * t1159 + f64x8::splat(400.0) * t1247 * t1159 + f64x8::splat(700.0) / f64x8::splat(3.0) * t1250 * t1159 + f64x8::splat(100.0) / f64x8::splat(9.0) * t1253 * t1159 + f64x8::splat(400.0) / f64x8::splat(9.0) * t1256 * t1159 + f64x8::splat(100.0) / f64x8::splat(3.0) * t1259 * t1159 + f64x8::splat(100.0) / f64x8::splat(9.0) * t1262 * t1264 + f64x8::splat(100.0) / f64x8::splat(9.0) * t1267 * t1264 - f64x8::splat(40.0) / f64x8::splat(9.0) * t383 * t1270;
            let t1275 = t1135 + t1180 + t1226 + t1273;
            let t1277 = t91 * t1099;
            let t1281 = t106 * t1086;
            let t1282 = t1281 * t1088;
            let t1283 = t90 * t1093;
            let t1287 = t181 * t552;
            let t1317 = -f64x8::splat(280.0) / f64x8::splat(9.0) * t520 * t1108 - f64x8::splat(320.0) / f64x8::splat(9.0) * t524 * t1108 - f64x8::splat(320.0) / f64x8::splat(9.0) * t527 * t1108 - f64x8::splat(40.0) * t531 * t1108 - f64x8::splat(40.0) * t534 * t1108 - f64x8::splat(400.0) / f64x8::splat(9.0) * t538 * t1108 - f64x8::splat(400.0) / f64x8::splat(9.0) * t541 * t1108 - f64x8::splat(440.0) / f64x8::splat(9.0) * t545 * t1108 - f64x8::splat(440.0) / f64x8::splat(9.0) * t548 * t1108 - f64x8::splat(40.0) / f64x8::splat(3.0) * t491 * t1108 - f64x8::splat(160.0) / f64x8::splat(9.0) * t495 * t1108 - f64x8::splat(160.0) / f64x8::splat(9.0) * t498 * t1108 - f64x8::splat(200.0) / f64x8::splat(9.0) * t502 * t1108;
            let t1334 = t210 * t176;
            let t1337 = t212 * t155;
            let t1338 = t1337 * t170;
            let t1341 = t537 * t176;
            let t1344 = t213 * t461;
            let t1347 = t215 * t161;
            let t1348 = t1347 * t176;
            let t1351 = t544 * t461;
            let t1354 = -f64x8::splat(200.0) / f64x8::splat(9.0) * t505 * t1108 - f64x8::splat(80.0) / f64x8::splat(3.0) * t509 * t1108 - f64x8::splat(80.0) / f64x8::splat(3.0) * t513 * t1108 - f64x8::splat(280.0) / f64x8::splat(9.0) * t517 * t1108 - f64x8::splat(40.0) / f64x8::splat(9.0) * t477 * t1108 - f64x8::splat(80.0) / f64x8::splat(9.0) * t481 * t1108 - f64x8::splat(80.0) / f64x8::splat(9.0) * t484 * t1108 - f64x8::splat(40.0) / f64x8::splat(3.0) * t488 * t1108 + f64x8::splat(500.0) * t1334 * t1159 + f64x8::splat(500.0) * t1338 * t1159 + f64x8::splat(10000.0) / f64x8::splat(9.0) * t1341 * t1159 + f64x8::splat(5500.0) / f64x8::splat(9.0) * t1344 * t1159 + f64x8::splat(5500.0) / f64x8::splat(9.0) * t1348 * t1159 + f64x8::splat(12100.0) / f64x8::splat(9.0) * t1351 * t1159;
            let t1356 = t216 * t1166;
            let t1359 = t198 * t152;
            let t1362 = t200 * t131;
            let t1363 = t1362 * t146;
            let t1366 = t508 * t152;
            let t1369 = t201 * t158;
            let t1372 = t203 * t137;
            let t1373 = t1372 * t152;
            let t1376 = t516 * t158;
            let t1379 = t204 * t164;
            let t1382 = t206 * t143;
            let t1383 = t1382 * t158;
            let t1386 = t523 * t164;
            let t1389 = t207 * t170;
            let t1392 = t209 * t149;
            let t1393 = t1392 * t164;
            let t1396 = t530 * t170;
            let t1399 = f64x8::splat(2200.0) / f64x8::splat(3.0) * t1356 * t1159 + f64x8::splat(500.0) / f64x8::splat(3.0) * t1359 * t1159 + f64x8::splat(500.0) / f64x8::splat(3.0) * t1363 * t1159 + f64x8::splat(400.0) * t1366 * t1159 + f64x8::splat(700.0) / f64x8::splat(3.0) * t1369 * t1159 + f64x8::splat(700.0) / f64x8::splat(3.0) * t1373 * t1159 + f64x8::splat(4900.0) / f64x8::splat(9.0) * t1376 * t1159 + f64x8::splat(2800.0) / f64x8::splat(9.0) * t1379 * t1159 + f64x8::splat(2800.0) / f64x8::splat(9.0) * t1383 * t1159 + f64x8::splat(6400.0) / f64x8::splat(9.0) * t1386 * t1159 + f64x8::splat(400.0) * t1389 * t1159 + f64x8::splat(400.0) * t1393 * t1159 + f64x8::splat(900.0) * t1396 * t1159;
            let t1400 = t186 * t128;
            let t1403 = t480 * t128;
            let t1406 = t189 * t134;
            let t1409 = t191 * t113;
            let t1410 = t1409 * t128;
            let t1413 = t487 * t134;
            let t1416 = t192 * t140;
            let t1419 = t194 * t119;
            let t1420 = t1419 * t134;
            let t1423 = t494 * t140;
            let t1426 = t195 * t146;
            let t1429 = t197 * t125;
            let t1430 = t1429 * t140;
            let t1433 = t501 * t146;
            let t1436 = t185 * t1154;
            let t1439 = t188 * t1154;
            let t1444 = f64x8::splat(100.0) / f64x8::splat(9.0) * t1400 * t1159 + f64x8::splat(400.0) / f64x8::splat(9.0) * t1403 * t1159 + f64x8::splat(100.0) / f64x8::splat(3.0) * t1406 * t1159 + f64x8::splat(100.0) / f64x8::splat(3.0) * t1410 * t1159 + f64x8::splat(100.0) * t1413 * t1159 + f64x8::splat(200.0) / f64x8::splat(3.0) * t1416 * t1159 + f64x8::splat(200.0) / f64x8::splat(3.0) * t1420 * t1159 + f64x8::splat(1600.0) / f64x8::splat(9.0) * t1423 * t1159 + f64x8::splat(1000.0) / f64x8::splat(9.0) * t1426 * t1159 + f64x8::splat(1000.0) / f64x8::splat(9.0) * t1430 * t1159 + f64x8::splat(2500.0) / f64x8::splat(9.0) * t1433 * t1159 + f64x8::splat(100.0) / f64x8::splat(9.0) * t1436 * t1264 + f64x8::splat(100.0) / f64x8::splat(9.0) * t1439 * t1264 - f64x8::splat(40.0) / f64x8::splat(9.0) * t474 * t1270;
            let t1446 = t1317 + t1354 + t1399 + t1444;
            let t1448 = -f64x8::splat(0.0015381393735744) * t1087 * t1089 * t1093 * t178 + f64x8::splat(0.05780898288) * t375 * t92 * t1099 * t178 - f64x8::splat(0.03153217248) * t375 * t92 * t378 * t466 + t103 * t1275 + f64x8::splat(0.05029721079103795) * t468 * t1277 * t470 - f64x8::splat(0.0006817668199851163) * t1282 * t1283 * t470 - f64x8::splat(0.027434842249657063) * t468 * t469 * t1287 + t183 * t1446;
            let t1465 = t252 * t461;
            let t1468 = t254 * t161;
            let t1469 = t1468 * t176;
            let t1472 = t631 * t461;
            let t1475 = t255 * t1166;
            let t1478 = t240 * t158;
            let t1481 = t242 * t137;
            let t1482 = t1481 * t152;
            let t1485 = t603 * t158;
            let t1488 = t243 * t164;
            let t1491 = t245 * t143;
            let t1492 = t1491 * t158;
            let t1495 = t610 * t164;
            let t1498 = t246 * t170;
            let t1501 = t248 * t149;
            let t1502 = t1501 * t164;
            let t1505 = t617 * t170;
            let t1508 = f64x8::splat(5500.0) / f64x8::splat(9.0) * t1465 * t1159 + f64x8::splat(5500.0) / f64x8::splat(9.0) * t1469 * t1159 + f64x8::splat(12100.0) / f64x8::splat(9.0) * t1472 * t1159 + f64x8::splat(2200.0) / f64x8::splat(3.0) * t1475 * t1159 + f64x8::splat(700.0) / f64x8::splat(3.0) * t1478 * t1159 + f64x8::splat(700.0) / f64x8::splat(3.0) * t1482 * t1159 + f64x8::splat(4900.0) / f64x8::splat(9.0) * t1485 * t1159 + f64x8::splat(2800.0) / f64x8::splat(9.0) * t1488 * t1159 + f64x8::splat(2800.0) / f64x8::splat(9.0) * t1492 * t1159 + f64x8::splat(6400.0) / f64x8::splat(9.0) * t1495 * t1159 + f64x8::splat(400.0) * t1498 * t1159 + f64x8::splat(400.0) * t1502 * t1159 + f64x8::splat(900.0) * t1505 * t1159;
            let t1509 = t249 * t176;
            let t1512 = t251 * t155;
            let t1513 = t1512 * t170;
            let t1516 = t624 * t176;
            let t1519 = t230 * t113;
            let t1520 = t1519 * t128;
            let t1523 = t574 * t134;
            let t1526 = t231 * t140;
            let t1529 = t233 * t119;
            let t1530 = t1529 * t134;
            let t1533 = t581 * t140;
            let t1536 = t234 * t146;
            let t1539 = t236 * t125;
            let t1540 = t1539 * t140;
            let t1543 = t588 * t146;
            let t1546 = t237 * t152;
            let t1549 = t239 * t131;
            let t1550 = t1549 * t146;
            let t1553 = t595 * t152;
            let t1556 = f64x8::splat(500.0) * t1509 * t1159 + f64x8::splat(500.0) * t1513 * t1159 + f64x8::splat(10000.0) / f64x8::splat(9.0) * t1516 * t1159 + f64x8::splat(100.0) / f64x8::splat(3.0) * t1520 * t1159 + f64x8::splat(100.0) * t1523 * t1159 + f64x8::splat(200.0) / f64x8::splat(3.0) * t1526 * t1159 + f64x8::splat(200.0) / f64x8::splat(3.0) * t1530 * t1159 + f64x8::splat(1600.0) / f64x8::splat(9.0) * t1533 * t1159 + f64x8::splat(1000.0) / f64x8::splat(9.0) * t1536 * t1159 + f64x8::splat(1000.0) / f64x8::splat(9.0) * t1540 * t1159 + f64x8::splat(2500.0) / f64x8::splat(9.0) * t1543 * t1159 + f64x8::splat(500.0) / f64x8::splat(3.0) * t1546 * t1159 + f64x8::splat(500.0) / f64x8::splat(3.0) * t1550 * t1159 + f64x8::splat(400.0) * t1553 * t1159;
            let t1584 = -f64x8::splat(40.0) * t618 * t1108 - f64x8::splat(40.0) * t621 * t1108 - f64x8::splat(400.0) / f64x8::splat(9.0) * t625 * t1108 - f64x8::splat(400.0) / f64x8::splat(9.0) * t628 * t1108 - f64x8::splat(440.0) / f64x8::splat(9.0) * t632 * t1108 - f64x8::splat(440.0) / f64x8::splat(9.0) * t635 * t1108 - f64x8::splat(160.0) / f64x8::splat(9.0) * t585 * t1108 - f64x8::splat(200.0) / f64x8::splat(9.0) * t589 * t1108 - f64x8::splat(200.0) / f64x8::splat(9.0) * t592 * t1108 - f64x8::splat(80.0) / f64x8::splat(3.0) * t596 * t1108 - f64x8::splat(80.0) / f64x8::splat(3.0) * t600 * t1108 - f64x8::splat(280.0) / f64x8::splat(9.0) * t604 * t1108 - f64x8::splat(280.0) / f64x8::splat(9.0) * t607 * t1108;
            let t1601 = t225 * t128;
            let t1604 = t567 * t128;
            let t1607 = t228 * t134;
            let t1610 = t224 * t1154;
            let t1613 = t227 * t1154;
            let t1618 = -f64x8::splat(320.0) / f64x8::splat(9.0) * t611 * t1108 - f64x8::splat(320.0) / f64x8::splat(9.0) * t614 * t1108 - f64x8::splat(40.0) / f64x8::splat(9.0) * t564 * t1108 - f64x8::splat(80.0) / f64x8::splat(9.0) * t568 * t1108 - f64x8::splat(80.0) / f64x8::splat(9.0) * t571 * t1108 - f64x8::splat(40.0) / f64x8::splat(3.0) * t575 * t1108 - f64x8::splat(40.0) / f64x8::splat(3.0) * t578 * t1108 - f64x8::splat(160.0) / f64x8::splat(9.0) * t582 * t1108 + f64x8::splat(100.0) / f64x8::splat(9.0) * t1601 * t1159 + f64x8::splat(400.0) / f64x8::splat(9.0) * t1604 * t1159 + f64x8::splat(100.0) / f64x8::splat(3.0) * t1607 * t1159 + f64x8::splat(100.0) / f64x8::splat(9.0) * t1610 * t1264 + f64x8::splat(100.0) / f64x8::splat(9.0) * t1613 * t1264 - f64x8::splat(40.0) / f64x8::splat(9.0) * t561 * t1270;
            let t1620 = t1508 + t1556 + t1584 + t1618;
            let t1628 = t181 * t723;
            let t1634 = t260 * t1154;
            let t1637 = t263 * t1154;
            let t1640 = t708 * t176;
            let t1643 = t288 * t461;
            let t1646 = t290 * t161;
            let t1647 = t1646 * t176;
            let t1650 = t715 * t461;
            let t1653 = t291 * t1166;
            let t1656 = t672 * t146;
            let t1659 = t273 * t152;
            let t1662 = t275 * t131;
            let t1663 = t1662 * t146;
            let t1666 = t679 * t152;
            let t1669 = t276 * t158;
            let t1672 = -f64x8::splat(40.0) / f64x8::splat(9.0) * t645 * t1270 + f64x8::splat(100.0) / f64x8::splat(9.0) * t1634 * t1264 + f64x8::splat(100.0) / f64x8::splat(9.0) * t1637 * t1264 + f64x8::splat(10000.0) / f64x8::splat(9.0) * t1640 * t1159 + f64x8::splat(5500.0) / f64x8::splat(9.0) * t1643 * t1159 + f64x8::splat(5500.0) / f64x8::splat(9.0) * t1647 * t1159 + f64x8::splat(12100.0) / f64x8::splat(9.0) * t1650 * t1159 + f64x8::splat(2200.0) / f64x8::splat(3.0) * t1653 * t1159 + f64x8::splat(2500.0) / f64x8::splat(9.0) * t1656 * t1159 + f64x8::splat(500.0) / f64x8::splat(3.0) * t1659 * t1159 + f64x8::splat(500.0) / f64x8::splat(3.0) * t1663 * t1159 + f64x8::splat(400.0) * t1666 * t1159 + f64x8::splat(700.0) / f64x8::splat(3.0) * t1669 * t1159;
            let t1673 = t278 * t137;
            let t1674 = t1673 * t152;
            let t1677 = t687 * t158;
            let t1680 = t279 * t164;
            let t1683 = t281 * t143;
            let t1684 = t1683 * t158;
            let t1687 = t694 * t164;
            let t1690 = t282 * t170;
            let t1693 = t261 * t128;
            let t1696 = t651 * t128;
            let t1699 = t264 * t134;
            let t1702 = t266 * t113;
            let t1703 = t1702 * t128;
            let t1706 = t658 * t134;
            let t1709 = t267 * t140;
            let t1712 = t269 * t119;
            let t1713 = t1712 * t134;
            let t1716 = t665 * t140;
            let t1719 = f64x8::splat(700.0) / f64x8::splat(3.0) * t1674 * t1159 + f64x8::splat(4900.0) / f64x8::splat(9.0) * t1677 * t1159 + f64x8::splat(2800.0) / f64x8::splat(9.0) * t1680 * t1159 + f64x8::splat(2800.0) / f64x8::splat(9.0) * t1684 * t1159 + f64x8::splat(6400.0) / f64x8::splat(9.0) * t1687 * t1159 + f64x8::splat(400.0) * t1690 * t1159 + f64x8::splat(100.0) / f64x8::splat(9.0) * t1693 * t1159 + f64x8::splat(400.0) / f64x8::splat(9.0) * t1696 * t1159 + f64x8::splat(100.0) / f64x8::splat(3.0) * t1699 * t1159 + f64x8::splat(100.0) / f64x8::splat(3.0) * t1703 * t1159 + f64x8::splat(100.0) * t1706 * t1159 + f64x8::splat(200.0) / f64x8::splat(3.0) * t1709 * t1159 + f64x8::splat(200.0) / f64x8::splat(3.0) * t1713 * t1159 + f64x8::splat(1600.0) / f64x8::splat(9.0) * t1716 * t1159;
            let t1721 = t270 * t146;
            let t1724 = t272 * t125;
            let t1725 = t1724 * t140;
            let t1746 = t284 * t149;
            let t1747 = t1746 * t164;
            let t1750 = t701 * t170;
            let t1753 = f64x8::splat(1000.0) / f64x8::splat(9.0) * t1721 * t1159 + f64x8::splat(1000.0) / f64x8::splat(9.0) * t1725 * t1159 - f64x8::splat(160.0) / f64x8::splat(9.0) * t666 * t1108 - f64x8::splat(160.0) / f64x8::splat(9.0) * t669 * t1108 - f64x8::splat(200.0) / f64x8::splat(9.0) * t673 * t1108 - f64x8::splat(200.0) / f64x8::splat(9.0) * t676 * t1108 - f64x8::splat(80.0) / f64x8::splat(3.0) * t680 * t1108 - f64x8::splat(80.0) / f64x8::splat(3.0) * t684 * t1108 - f64x8::splat(40.0) / f64x8::splat(9.0) * t648 * t1108 - f64x8::splat(80.0) / f64x8::splat(9.0) * t652 * t1108 - f64x8::splat(80.0) / f64x8::splat(9.0) * t655 * t1108 + f64x8::splat(400.0) * t1747 * t1159 + f64x8::splat(900.0) * t1750 * t1159;
            let t1754 = t285 * t176;
            let t1757 = t287 * t155;
            let t1758 = t1757 * t170;
            let t1785 = f64x8::splat(500.0) * t1754 * t1159 + f64x8::splat(500.0) * t1758 * t1159 - f64x8::splat(440.0) / f64x8::splat(9.0) * t719 * t1108 - f64x8::splat(280.0) / f64x8::splat(9.0) * t688 * t1108 - f64x8::splat(280.0) / f64x8::splat(9.0) * t691 * t1108 - f64x8::splat(320.0) / f64x8::splat(9.0) * t695 * t1108 - f64x8::splat(320.0) / f64x8::splat(9.0) * t698 * t1108 - f64x8::splat(40.0) * t702 * t1108 - f64x8::splat(40.0) * t705 * t1108 - f64x8::splat(400.0) / f64x8::splat(9.0) * t709 * t1108 - f64x8::splat(400.0) / f64x8::splat(9.0) * t712 * t1108 - f64x8::splat(440.0) / f64x8::splat(9.0) * t716 * t1108 - f64x8::splat(40.0) / f64x8::splat(3.0) * t659 * t1108 - f64x8::splat(40.0) / f64x8::splat(3.0) * t662 * t1108;
            let t1787 = t1672 + t1719 + t1753 + t1785;
            let t1789 = -f64x8::splat(0.0015381393735744) * t1087 * t1089 * t1093 * t257 + f64x8::splat(0.05780898288) * t375 * t92 * t1099 * t257 - f64x8::splat(0.03153217248) * t375 * t92 * t378 * t639 + t103 * t1620 + f64x8::splat(0.05029721079103795) * t468 * t1277 * t641 - f64x8::splat(0.0006817668199851163) * t1282 * t1283 * t641 - f64x8::splat(0.027434842249657063) * t468 * t469 * t1628 + t183 * t1787;
            let t1791 = t1078 * t220 - t1078 * t295 + t83 * t1448 + t222 * t1789 + f64x8::splat(2.0) * t370 * t554 - f64x8::splat(2.0) * t370 * t725;
            let t1796 = ((t3).select(f64x8::splat(0.0), t7 * t981 * t297 / f64x8::splat(12.0) - t7 * t303 * t727 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1791));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t1796 + f64x8::splat(4.0) * t732;
            acc_v2rho2 = tv2rho20;
            let t1803 = t1090 * t93;
            let t1805 = f64x8::splat(1.0) / t19 / t1803;
            let t1806 = t90 * t1805;
            let t1807 = t178 * v_sigma;
            let t1817 = t378 * t181;
            let t1821 = t1281 * t90;
            let t1822 = t1805 * v_sigma;
            let t1829 = f64x8::splat(0.0005768022650904) * t1087 * t1806 * t1807 - f64x8::splat(0.01576608624) * t375 * t469 * t178 + f64x8::splat(0.00591228234) * t375 * t384 * t466 - f64x8::splat(0.013717421124828532) * t738 * t1817 * t218 + f64x8::splat(0.00025566255749441856) * t1821 * t1822 * t470 + f64x8::splat(0.0051440329218107) * t738 * t739 * t552;
            let t1832 = t257 * v_sigma;
            let t1851 = f64x8::splat(0.0005768022650904) * t1087 * t1806 * t1832 - f64x8::splat(0.01576608624) * t375 * t469 * t257 + f64x8::splat(0.00591228234) * t375 * t384 * t639 - f64x8::splat(0.013717421124828532) * t738 * t1817 * t293 + f64x8::splat(0.00025566255749441856) * t1821 * t1822 * t641 + f64x8::splat(0.0051440329218107) * t738 * t739 * t723;
            let t1853 = t83 * t1829 + t222 * t1851 + t370 * t743 - t370 * t751;
            let t1858 = ((t3).select(f64x8::splat(0.0), -t7 * t303 * t753 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t1853));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t1858 + f64x8::splat(2.0) * t757;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t1869 = t798 * t96;
            let t1872 = t762 * t96;
            let t1877 = t767 * t96;
            let t1882 = t773 * t96;
            let t1887 = t779 * t96;
            let t1890 = t105 * t90;
            let t1892 = f64x8::splat(1.0) / t19 / t1090;
            let t1893 = t1892 * t122;
            let t1894 = t1893 * v_tau;
            let t1897 = t118 * v_tau;
            let t1898 = t90 * t1892;
            let t1899 = t1898 * t122;
            let t1902 = t822 * t96;
            let t1907 = t804 * t96;
            let t1910 = f64x8::splat(35.0) / f64x8::splat(3.0) * t150 * t1869 + f64x8::splat(5.0) / f64x8::splat(3.0) * t114 * t1872 + f64x8::splat(10.0) / f64x8::splat(3.0) * t392 * t1872 + f64x8::splat(10.0) / f64x8::splat(3.0) * t120 * t1877 + f64x8::splat(5.0) * t399 * t1877 + f64x8::splat(5.0) * t126 * t1882 + f64x8::splat(20.0) / f64x8::splat(3.0) * t406 * t1882 + f64x8::splat(20.0) / f64x8::splat(3.0) * t132 * t1887 - f64x8::splat(20.0) / f64x8::splat(3.0) * t1890 * t1894 - f64x8::splat(20.0) / f64x8::splat(3.0) * t1897 * t1899 + f64x8::splat(55.0) / f64x8::splat(3.0) * t174 * t1902 + f64x8::splat(40.0) / f64x8::splat(3.0) * t435 * t1869 + f64x8::splat(40.0) / f64x8::splat(3.0) * t156 * t1907;
            let t1913 = t810 * t96;
            let t1918 = t816 * t96;
            let t1925 = t785 * t96;
            let t1930 = t792 * t96;
            let t1935 = t96 * t116;
            let t1938 = t1898 * v_tau;
            let t1945 = f64x8::splat(15.0) * t442 * t1907 + f64x8::splat(15.0) * t162 * t1913 + f64x8::splat(50.0) / f64x8::splat(3.0) * t449 * t1913 + f64x8::splat(50.0) / f64x8::splat(3.0) * t168 * t1918 + f64x8::splat(55.0) / f64x8::splat(3.0) * t456 * t1918 + f64x8::splat(25.0) / f64x8::splat(3.0) * t413 * t1887 + f64x8::splat(25.0) / f64x8::splat(3.0) * t138 * t1925 + f64x8::splat(10.0) * t420 * t1925 + f64x8::splat(10.0) * t144 * t1930 + f64x8::splat(35.0) / f64x8::splat(3.0) * t428 * t1930 + f64x8::splat(5.0) / f64x8::splat(3.0) * t759 * t1935 - f64x8::splat(200.0) / f64x8::splat(3.0) * t1230 * t1938 - f64x8::splat(1100.0) / f64x8::splat(3.0) * t1209 * t1938 - f64x8::splat(1100.0) / f64x8::splat(3.0) * t1153 * t1938;
            let t1973 = -f64x8::splat(2420.0) / f64x8::splat(3.0) * t1162 * t1938 - f64x8::splat(200.0) / f64x8::splat(3.0) * t1234 * t1938 - f64x8::splat(500.0) / f64x8::splat(3.0) * t1237 * t1938 - f64x8::splat(100.0) * t1240 * t1938 - f64x8::splat(100.0) * t1244 * t1938 - f64x8::splat(240.0) * t1247 * t1938 - f64x8::splat(140.0) * t1250 * t1938 - f64x8::splat(140.0) * t1171 * t1938 - f64x8::splat(980.0) / f64x8::splat(3.0) * t1174 * t1938 - f64x8::splat(560.0) / f64x8::splat(3.0) * t1177 * t1938 - f64x8::splat(20.0) / f64x8::splat(3.0) * t1253 * t1938 - f64x8::splat(80.0) / f64x8::splat(3.0) * t1256 * t1938 - f64x8::splat(20.0) * t1259 * t1938;
            let t2002 = -f64x8::splat(20.0) * t1213 * t1938 - f64x8::splat(60.0) * t1216 * t1938 - f64x8::splat(40.0) * t1219 * t1938 - f64x8::splat(40.0) * t1223 * t1938 - f64x8::splat(320.0) / f64x8::splat(3.0) * t1227 * t1938 - f64x8::splat(440.0) * t1167 * t1938 - f64x8::splat(560.0) / f64x8::splat(3.0) * t1183 * t1938 - f64x8::splat(1280.0) / f64x8::splat(3.0) * t1186 * t1938 - f64x8::splat(240.0) * t1189 * t1938 - f64x8::splat(240.0) * t1193 * t1938 - f64x8::splat(540.0) * t1196 * t1938 - f64x8::splat(300.0) * t1199 * t1938 - f64x8::splat(300.0) * t1203 * t1938 - f64x8::splat(2000.0) / f64x8::splat(3.0) * t1206 * t1938;
            let t2004 = t1910 + t1945 + t1973 + t2002;
            let t2006 = t181 * t874;
            let t2026 = t185 * t90;
            let t2029 = t188 * v_tau;
            let t2038 = f64x8::splat(10.0) * t508 * t1925 + f64x8::splat(10.0) * t201 * t1930 + f64x8::splat(35.0) / f64x8::splat(3.0) * t516 * t1930 + f64x8::splat(5.0) / f64x8::splat(3.0) * t186 * t1872 + f64x8::splat(10.0) / f64x8::splat(3.0) * t480 * t1872 + f64x8::splat(10.0) / f64x8::splat(3.0) * t189 * t1877 + f64x8::splat(5.0) * t487 * t1877 + f64x8::splat(5.0) * t192 * t1882 - f64x8::splat(20.0) / f64x8::splat(3.0) * t2026 * t1894 - f64x8::splat(20.0) / f64x8::splat(3.0) * t2029 * t1899 + f64x8::splat(50.0) / f64x8::splat(3.0) * t213 * t1918 + f64x8::splat(55.0) / f64x8::splat(3.0) * t544 * t1918 + f64x8::splat(55.0) / f64x8::splat(3.0) * t216 * t1902;
            let t2067 = f64x8::splat(35.0) / f64x8::splat(3.0) * t204 * t1869 + f64x8::splat(40.0) / f64x8::splat(3.0) * t523 * t1869 + f64x8::splat(40.0) / f64x8::splat(3.0) * t207 * t1907 + f64x8::splat(15.0) * t530 * t1907 + f64x8::splat(15.0) * t210 * t1913 + f64x8::splat(50.0) / f64x8::splat(3.0) * t537 * t1913 + f64x8::splat(20.0) / f64x8::splat(3.0) * t494 * t1882 + f64x8::splat(20.0) / f64x8::splat(3.0) * t195 * t1887 + f64x8::splat(25.0) / f64x8::splat(3.0) * t501 * t1887 + f64x8::splat(25.0) / f64x8::splat(3.0) * t198 * t1925 + f64x8::splat(5.0) / f64x8::splat(3.0) * t829 * t1935 - f64x8::splat(100.0) * t1359 * t1938 - f64x8::splat(100.0) * t1363 * t1938 - f64x8::splat(240.0) * t1366 * t1938;
            let t2095 = -f64x8::splat(140.0) * t1369 * t1938 - f64x8::splat(140.0) * t1373 * t1938 - f64x8::splat(20.0) / f64x8::splat(3.0) * t1400 * t1938 - f64x8::splat(80.0) / f64x8::splat(3.0) * t1403 * t1938 - f64x8::splat(20.0) * t1406 * t1938 - f64x8::splat(20.0) * t1410 * t1938 - f64x8::splat(60.0) * t1413 * t1938 - f64x8::splat(40.0) * t1416 * t1938 - f64x8::splat(2420.0) / f64x8::splat(3.0) * t1351 * t1938 - f64x8::splat(440.0) * t1356 * t1938 - f64x8::splat(980.0) / f64x8::splat(3.0) * t1376 * t1938 - f64x8::splat(560.0) / f64x8::splat(3.0) * t1379 * t1938 - f64x8::splat(560.0) / f64x8::splat(3.0) * t1383 * t1938;
            let t2124 = -f64x8::splat(1280.0) / f64x8::splat(3.0) * t1386 * t1938 - f64x8::splat(240.0) * t1389 * t1938 - f64x8::splat(240.0) * t1393 * t1938 - f64x8::splat(540.0) * t1396 * t1938 - f64x8::splat(300.0) * t1334 * t1938 - f64x8::splat(300.0) * t1338 * t1938 - f64x8::splat(2000.0) / f64x8::splat(3.0) * t1341 * t1938 - f64x8::splat(40.0) * t1420 * t1938 - f64x8::splat(320.0) / f64x8::splat(3.0) * t1423 * t1938 - f64x8::splat(200.0) / f64x8::splat(3.0) * t1426 * t1938 - f64x8::splat(200.0) / f64x8::splat(3.0) * t1430 * t1938 - f64x8::splat(500.0) / f64x8::splat(3.0) * t1433 * t1938 - f64x8::splat(1100.0) / f64x8::splat(3.0) * t1344 * t1938 - f64x8::splat(1100.0) / f64x8::splat(3.0) * t1348 * t1938;
            let t2126 = t2038 + t2067 + t2095 + t2124;
            let t2128 = -f64x8::splat(0.01576608624) * t375 * t92 * t378 * t827 + t103 * t2004 - f64x8::splat(0.013717421124828532) * t468 * t469 * t2006 + t183 * t2126;
            let t2135 = t224 * t90;
            let t2138 = t227 * v_tau;
            let t2163 = -f64x8::splat(20.0) / f64x8::splat(3.0) * t2135 * t1894 - f64x8::splat(20.0) / f64x8::splat(3.0) * t2138 * t1899 + f64x8::splat(15.0) * t249 * t1913 + f64x8::splat(50.0) / f64x8::splat(3.0) * t624 * t1913 + f64x8::splat(50.0) / f64x8::splat(3.0) * t252 * t1918 + f64x8::splat(55.0) / f64x8::splat(3.0) * t631 * t1918 + f64x8::splat(55.0) / f64x8::splat(3.0) * t255 * t1902 + f64x8::splat(10.0) * t240 * t1930 + f64x8::splat(35.0) / f64x8::splat(3.0) * t603 * t1930 + f64x8::splat(35.0) / f64x8::splat(3.0) * t243 * t1869 + f64x8::splat(40.0) / f64x8::splat(3.0) * t610 * t1869 + f64x8::splat(40.0) / f64x8::splat(3.0) * t246 * t1907 + f64x8::splat(15.0) * t617 * t1907;
            let t2192 = f64x8::splat(5.0) * t574 * t1877 - f64x8::splat(320.0) / f64x8::splat(3.0) * t1533 * t1938 - f64x8::splat(200.0) / f64x8::splat(3.0) * t1536 * t1938 - f64x8::splat(200.0) / f64x8::splat(3.0) * t1540 * t1938 - f64x8::splat(500.0) / f64x8::splat(3.0) * t1543 * t1938 - f64x8::splat(100.0) * t1546 * t1938 - f64x8::splat(100.0) * t1550 * t1938 - f64x8::splat(240.0) * t1553 * t1938 - f64x8::splat(20.0) / f64x8::splat(3.0) * t1601 * t1938 - f64x8::splat(80.0) / f64x8::splat(3.0) * t1604 * t1938 - f64x8::splat(20.0) * t1607 * t1938 - f64x8::splat(1100.0) / f64x8::splat(3.0) * t1469 * t1938 - f64x8::splat(2420.0) / f64x8::splat(3.0) * t1472 * t1938 - f64x8::splat(440.0) * t1475 * t1938;
            let t2220 = -f64x8::splat(140.0) * t1478 * t1938 - f64x8::splat(140.0) * t1482 * t1938 - f64x8::splat(980.0) / f64x8::splat(3.0) * t1485 * t1938 - f64x8::splat(560.0) / f64x8::splat(3.0) * t1488 * t1938 - f64x8::splat(560.0) / f64x8::splat(3.0) * t1492 * t1938 - f64x8::splat(1280.0) / f64x8::splat(3.0) * t1495 * t1938 - f64x8::splat(240.0) * t1498 * t1938 - f64x8::splat(240.0) * t1502 * t1938 - f64x8::splat(540.0) * t1505 * t1938 - f64x8::splat(20.0) * t1520 * t1938 - f64x8::splat(60.0) * t1523 * t1938 - f64x8::splat(40.0) * t1526 * t1938 - f64x8::splat(40.0) * t1530 * t1938;
            let t2249 = -f64x8::splat(300.0) * t1509 * t1938 - f64x8::splat(300.0) * t1513 * t1938 - f64x8::splat(2000.0) / f64x8::splat(3.0) * t1516 * t1938 - f64x8::splat(1100.0) / f64x8::splat(3.0) * t1465 * t1938 + f64x8::splat(5.0) / f64x8::splat(3.0) * t878 * t1935 + f64x8::splat(5.0) * t231 * t1882 + f64x8::splat(20.0) / f64x8::splat(3.0) * t581 * t1882 + f64x8::splat(20.0) / f64x8::splat(3.0) * t234 * t1887 + f64x8::splat(25.0) / f64x8::splat(3.0) * t588 * t1887 + f64x8::splat(25.0) / f64x8::splat(3.0) * t237 * t1925 + f64x8::splat(10.0) * t595 * t1925 + f64x8::splat(5.0) / f64x8::splat(3.0) * t225 * t1872 + f64x8::splat(10.0) / f64x8::splat(3.0) * t567 * t1872 + f64x8::splat(10.0) / f64x8::splat(3.0) * t228 * t1877;
            let t2251 = t2163 + t2192 + t2220 + t2249;
            let t2253 = t181 * t970;
            let t2283 = -f64x8::splat(80.0) / f64x8::splat(3.0) * t1696 * t1938 - f64x8::splat(20.0) * t1699 * t1938 - f64x8::splat(20.0) * t1703 * t1938 - f64x8::splat(60.0) * t1706 * t1938 - f64x8::splat(40.0) * t1709 * t1938 - f64x8::splat(40.0) * t1713 * t1938 - f64x8::splat(320.0) / f64x8::splat(3.0) * t1716 * t1938 - f64x8::splat(200.0) / f64x8::splat(3.0) * t1721 * t1938 - f64x8::splat(200.0) / f64x8::splat(3.0) * t1725 * t1938 - f64x8::splat(500.0) / f64x8::splat(3.0) * t1656 * t1938 - f64x8::splat(20.0) / f64x8::splat(3.0) * t1693 * t1938 - f64x8::splat(440.0) * t1653 * t1938 - f64x8::splat(100.0) * t1659 * t1938;
            let t2312 = -f64x8::splat(100.0) * t1663 * t1938 - f64x8::splat(240.0) * t1666 * t1938 - f64x8::splat(140.0) * t1669 * t1938 - f64x8::splat(140.0) * t1674 * t1938 - f64x8::splat(980.0) / f64x8::splat(3.0) * t1677 * t1938 - f64x8::splat(560.0) / f64x8::splat(3.0) * t1680 * t1938 - f64x8::splat(560.0) / f64x8::splat(3.0) * t1684 * t1938 - f64x8::splat(1280.0) / f64x8::splat(3.0) * t1687 * t1938 - f64x8::splat(240.0) * t1690 * t1938 - f64x8::splat(240.0) * t1747 * t1938 - f64x8::splat(540.0) * t1750 * t1938 - f64x8::splat(300.0) * t1754 * t1938 - f64x8::splat(300.0) * t1758 * t1938 - f64x8::splat(2000.0) / f64x8::splat(3.0) * t1640 * t1938;
            let t2340 = -f64x8::splat(1100.0) / f64x8::splat(3.0) * t1643 * t1938 - f64x8::splat(1100.0) / f64x8::splat(3.0) * t1647 * t1938 - f64x8::splat(2420.0) / f64x8::splat(3.0) * t1650 * t1938 + f64x8::splat(5.0) / f64x8::splat(3.0) * t925 * t1935 + f64x8::splat(15.0) * t701 * t1907 + f64x8::splat(15.0) * t285 * t1913 + f64x8::splat(50.0) / f64x8::splat(3.0) * t708 * t1913 + f64x8::splat(50.0) / f64x8::splat(3.0) * t288 * t1918 + f64x8::splat(55.0) / f64x8::splat(3.0) * t715 * t1918 + f64x8::splat(55.0) / f64x8::splat(3.0) * t291 * t1902 + f64x8::splat(25.0) / f64x8::splat(3.0) * t273 * t1925 + f64x8::splat(10.0) * t679 * t1925 + f64x8::splat(10.0) * t276 * t1930;
            let t2349 = t260 * t90;
            let t2352 = t263 * v_tau;
            let t2371 = f64x8::splat(35.0) / f64x8::splat(3.0) * t687 * t1930 + f64x8::splat(35.0) / f64x8::splat(3.0) * t279 * t1869 + f64x8::splat(40.0) / f64x8::splat(3.0) * t694 * t1869 + f64x8::splat(40.0) / f64x8::splat(3.0) * t282 * t1907 - f64x8::splat(20.0) / f64x8::splat(3.0) * t2349 * t1894 - f64x8::splat(20.0) / f64x8::splat(3.0) * t2352 * t1899 + f64x8::splat(10.0) / f64x8::splat(3.0) * t264 * t1877 + f64x8::splat(5.0) * t658 * t1877 + f64x8::splat(5.0) * t267 * t1882 + f64x8::splat(20.0) / f64x8::splat(3.0) * t665 * t1882 + f64x8::splat(20.0) / f64x8::splat(3.0) * t270 * t1887 + f64x8::splat(25.0) / f64x8::splat(3.0) * t672 * t1887 + f64x8::splat(5.0) / f64x8::splat(3.0) * t261 * t1872 + f64x8::splat(10.0) / f64x8::splat(3.0) * t651 * t1872;
            let t2373 = t2283 + t2312 + t2340 + t2371;
            let t2375 = -f64x8::splat(0.01576608624) * t375 * t92 * t378 * t923 + t103 * t2251 - f64x8::splat(0.013717421124828532) * t468 * t469 * t2253 + t183 * t2373;
            let t2377 = t83 * t2128 + t222 * t2375 + t370 * t876 - t370 * t972;
            let t2382 = ((t3).select(f64x8::splat(0.0), -t7 * t303 * t974 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t2377));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t2382 + f64x8::splat(2.0) * t978;
            acc_v2rhotau = tv2rhotau0;
            let t2388 = t1158 * t181;
            let t2392 = -f64x8::splat(0.0002163008494089) * t1087 * t1263 * t178 - f64x8::splat(9.587345906040697e-05) * t1821 * t2388 * t218;
            let t2400 = -f64x8::splat(0.0002163008494089) * t1087 * t1263 * t257 - f64x8::splat(9.587345906040697e-05) * t1821 * t2388 * t293;
            let t2402 = t222 * t2400 + t83 * t2392;
            let t2406 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t2402));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t2406;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t2414 = f64x8::splat(0.00591228234) * t375 * t384 * t827 + f64x8::splat(0.0051440329218107) * t738 * t739 * t874;
            let t2422 = f64x8::splat(0.00591228234) * t375 * t384 * t923 + f64x8::splat(0.0051440329218107) * t738 * t739 * t970;
            let t2424 = t222 * t2422 + t83 * t2414;
            let t2428 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t2424));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t2428;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t2431 = f64x8::splat(1.0) / t19 / t376;
            let t2432 = t2431 * t122;
            let t2435 = t118 * t90;
            let t2438 = t128 * t90;
            let t2439 = t2438 * t2431;
            let t2442 = t134 * t90;
            let t2443 = t2442 * t2431;
            let t2450 = t140 * t90;
            let t2451 = t2450 * t2431;
            let t2458 = t146 * t90;
            let t2459 = t2458 * t2431;
            let t2466 = t152 * t90;
            let t2467 = t2466 * t2431;
            let t2474 = t158 * t90;
            let t2475 = t2474 * t2431;
            let t2478 = f64x8::splat(12.0) * t120 * t2443 + f64x8::splat(12.0) * t1212 * t2439 + f64x8::splat(24.0) * t1222 * t2443 + f64x8::splat(40.0) * t1233 * t2451 + f64x8::splat(60.0) * t1243 * t2459 + f64x8::splat(24.0) * t126 * t2451 + f64x8::splat(40.0) * t132 * t2459 + f64x8::splat(60.0) * t138 * t2467 + f64x8::splat(84.0) * t144 * t2475 + f64x8::splat(4.0) * t1890 * t2432 + f64x8::splat(4.0) * t2435 * t2432 + f64x8::splat(16.0) * t392 * t2439 + f64x8::splat(36.0) * t399 * t2443 + f64x8::splat(64.0) * t406 * t2451 + f64x8::splat(100.0) * t413 * t2459 + f64x8::splat(144.0) * t420 * t2467;
            let t2483 = t164 * t90;
            let t2484 = t2483 * t2431;
            let t2491 = t170 * t90;
            let t2492 = t2491 * t2431;
            let t2499 = t176 * t90;
            let t2500 = t2499 * t2431;
            let t2509 = t461 * t90;
            let t2510 = t2509 * t2431;
            let t2517 = t1166 * t90;
            let t2518 = t2517 * t2431;
            let t2521 = f64x8::splat(4.0) * t114 * t2439 + f64x8::splat(220.0) * t1152 * t2500 + f64x8::splat(84.0) * t1170 * t2467 + f64x8::splat(112.0) * t1182 * t2475 + f64x8::splat(144.0) * t1192 * t2484 + f64x8::splat(180.0) * t1202 * t2492 + f64x8::splat(112.0) * t150 * t2484 + f64x8::splat(144.0) * t156 * t2492 + f64x8::splat(180.0) * t162 * t2500 + f64x8::splat(220.0) * t168 * t2510 + f64x8::splat(264.0) * t174 * t2518 + f64x8::splat(196.0) * t428 * t2475 + f64x8::splat(256.0) * t435 * t2484 + f64x8::splat(324.0) * t442 * t2492 + f64x8::splat(400.0) * t449 * t2500 + f64x8::splat(484.0) * t456 * t2510;
            let t2522 = t2478 + t2521;
            let t2526 = t188 * t90;
            let t2557 = f64x8::splat(180.0) * t1337 * t2492 + f64x8::splat(144.0) * t1392 * t2484 + f64x8::splat(12.0) * t1409 * t2439 + f64x8::splat(24.0) * t1419 * t2443 + f64x8::splat(4.0) * t186 * t2439 + f64x8::splat(12.0) * t189 * t2443 + f64x8::splat(24.0) * t192 * t2451 + f64x8::splat(40.0) * t195 * t2459 + f64x8::splat(4.0) * t2026 * t2432 + f64x8::splat(180.0) * t210 * t2500 + f64x8::splat(4.0) * t2526 * t2432 + f64x8::splat(16.0) * t480 * t2439 + f64x8::splat(36.0) * t487 * t2443 + f64x8::splat(64.0) * t494 * t2451 + f64x8::splat(324.0) * t530 * t2492 + f64x8::splat(400.0) * t537 * t2500;
            let t2590 = f64x8::splat(220.0) * t1347 * t2500 + f64x8::splat(60.0) * t1362 * t2459 + f64x8::splat(84.0) * t1372 * t2467 + f64x8::splat(112.0) * t1382 * t2475 + f64x8::splat(40.0) * t1429 * t2451 + f64x8::splat(60.0) * t198 * t2467 + f64x8::splat(84.0) * t201 * t2475 + f64x8::splat(112.0) * t204 * t2484 + f64x8::splat(144.0) * t207 * t2492 + f64x8::splat(220.0) * t213 * t2510 + f64x8::splat(264.0) * t216 * t2518 + f64x8::splat(100.0) * t501 * t2459 + f64x8::splat(144.0) * t508 * t2467 + f64x8::splat(196.0) * t516 * t2475 + f64x8::splat(256.0) * t523 * t2484 + f64x8::splat(484.0) * t544 * t2510;
            let t2591 = t2557 + t2590;
            let t2593 = t103 * t2522 + t183 * t2591;
            let t2597 = t227 * t90;
            let t2628 = f64x8::splat(112.0) * t1491 * t2475 + f64x8::splat(144.0) * t1501 * t2484 + f64x8::splat(180.0) * t1512 * t2492 + f64x8::splat(12.0) * t1519 * t2439 + f64x8::splat(4.0) * t2135 * t2432 + f64x8::splat(4.0) * t225 * t2439 + f64x8::splat(12.0) * t228 * t2443 + f64x8::splat(112.0) * t243 * t2484 + f64x8::splat(4.0) * t2597 * t2432 + f64x8::splat(16.0) * t567 * t2439 + f64x8::splat(144.0) * t246 * t2492 + f64x8::splat(256.0) * t610 * t2484 + f64x8::splat(180.0) * t249 * t2500 + f64x8::splat(324.0) * t617 * t2492 + f64x8::splat(400.0) * t624 * t2500 + f64x8::splat(220.0) * t252 * t2510;
            let t2661 = f64x8::splat(220.0) * t1468 * t2500 + f64x8::splat(84.0) * t1481 * t2467 + f64x8::splat(24.0) * t1529 * t2443 + f64x8::splat(40.0) * t1539 * t2451 + f64x8::splat(60.0) * t1549 * t2459 + f64x8::splat(24.0) * t231 * t2451 + f64x8::splat(40.0) * t234 * t2459 + f64x8::splat(60.0) * t237 * t2467 + f64x8::splat(84.0) * t240 * t2475 + f64x8::splat(36.0) * t574 * t2443 + f64x8::splat(64.0) * t581 * t2451 + f64x8::splat(100.0) * t588 * t2459 + f64x8::splat(144.0) * t595 * t2467 + f64x8::splat(196.0) * t603 * t2475 + f64x8::splat(484.0) * t631 * t2510 + f64x8::splat(264.0) * t255 * t2518;
            let t2662 = t2628 + t2661;
            let t2666 = t263 * t90;
            let t2697 = f64x8::splat(220.0) * t1646 * t2500 + f64x8::splat(12.0) * t1702 * t2439 + f64x8::splat(24.0) * t1712 * t2443 + f64x8::splat(180.0) * t1757 * t2492 + f64x8::splat(4.0) * t2349 * t2432 + f64x8::splat(4.0) * t2666 * t2432 + f64x8::splat(4.0) * t261 * t2439 + f64x8::splat(16.0) * t651 * t2439 + f64x8::splat(12.0) * t264 * t2443 + f64x8::splat(36.0) * t658 * t2443 + f64x8::splat(24.0) * t267 * t2451 + f64x8::splat(64.0) * t665 * t2451 + f64x8::splat(40.0) * t270 * t2459 + f64x8::splat(400.0) * t708 * t2500 + f64x8::splat(220.0) * t288 * t2510 + f64x8::splat(484.0) * t715 * t2510;
            let t2730 = f64x8::splat(60.0) * t1662 * t2459 + f64x8::splat(84.0) * t1673 * t2467 + f64x8::splat(112.0) * t1683 * t2475 + f64x8::splat(40.0) * t1724 * t2451 + f64x8::splat(144.0) * t1746 * t2484 + f64x8::splat(100.0) * t672 * t2459 + f64x8::splat(60.0) * t273 * t2467 + f64x8::splat(144.0) * t679 * t2467 + f64x8::splat(84.0) * t276 * t2475 + f64x8::splat(196.0) * t687 * t2475 + f64x8::splat(112.0) * t279 * t2484 + f64x8::splat(256.0) * t694 * t2484 + f64x8::splat(144.0) * t282 * t2492 + f64x8::splat(324.0) * t701 * t2492 + f64x8::splat(180.0) * t285 * t2500 + f64x8::splat(264.0) * t291 * t2518;
            let t2731 = t2697 + t2730;
            let t2733 = t103 * t2662 + t183 * t2731;
            let t2735 = t222 * t2733 + t83 * t2593;
            let t2739 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t2735));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t2739;
            acc_v2tau2 = tv2tau20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2rholapl, ip, m, acc_v2rholapl);
        store_add(v2rhotau, ip, m, acc_v2rhotau);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v2sigmalapl, ip, m, acc_v2sigmalapl);
        store_add(v2sigmatau, ip, m, acc_v2sigmatau);
        store_add(v2lapl2, ip, m, acc_v2lapl2);
        store_add(v2lapltau, ip, m, acc_v2lapltau);
        store_add(v2tau2, ip, m, acc_v2tau2);
        ip += 8;
    }
}

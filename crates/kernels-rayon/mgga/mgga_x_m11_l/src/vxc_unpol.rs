//! MGGA_X_M11_L vxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_m11_l_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

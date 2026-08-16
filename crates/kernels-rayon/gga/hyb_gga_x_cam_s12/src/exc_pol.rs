//! HYB_GGA_X_CAM_S12 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/hyb_gga_x_cam_s12.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_gga_x_cam_s12_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    param_hyb_coeff_0: f64,
    param_hyb_coeff_1: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
        let t28 = param_C * sigma0;
        let t29 = rho0 * rho0;
        let t30 = pow_1_3(rho0);
        let t31 = t30 * t30;
        let t33 = 1.0 / t31 / t29;
        let t35 = sigma0 * sigma0;
        let t36 = param_D * t35;
        let t37 = t29 * t29;
        let t38 = t37 * rho0;
        let t40 = 1.0 / t30 / t38;
        let t42 = t28 * t33 + t36 * t40 + 1.0;
        let t45 = param_B * (1.0 - 1.0 / t42);
        let t46 = param_E * sigma0;
        let t48 = t46 * t33 + 1.0;
        let t50 = 1.0 - 1.0 / t48;
        let t52 = t45 * t50 + param_A;
        let t53 = t27 * t52;
        let t54 = t2 * t2;
        let t55 = M_PI * t54;
        let t56 = 1.0 / M_PI;
        let t57 = pow_1_3(t56);
        let t58 = 1.0 / t57;
        let t59 = M_CBRT4;
        let t60 = t58 * t59;
        let t63 = t55 * t60 / t52;
        let t64 = f64::sqrt(t63);
        let t66 = param_hyb_omega_0 / t64;
        let t67 = M_CBRT2;
        let t68 = t19 * t6;
        let t69 = pow_1_3(t68);
        let t70 = 1.0 / t69;
        let t71 = t67 * t70;
        let t73 = t66 * t71 / 2.0;
        let t74 = 0.135e1 <= t73;
        let t75 = 0.135e1 < t73;
        let t76 = piecewise3(t75, t73, 0.135e1);
        let t77 = t76 * t76;
        let t80 = t77 * t77;
        let t81 = 1.0 / t80;
        let t83 = t80 * t77;
        let t84 = 1.0 / t83;
        let t86 = t80 * t80;
        let t87 = 1.0 / t86;
        let t90 = 1.0 / t86 / t77;
        let t93 = 1.0 / t86 / t80;
        let t96 = 1.0 / t86 / t83;
        let t98 = t86 * t86;
        let t99 = 1.0 / t98;
        let t102 = piecewise3(t75, 0.135e1, t73);
        let t103 = f64::sqrt(M_PI);
        let t104 = 1.0 / t102;
        let t106 = erf_approx(t104 / 2.0);
        let t108 = t102 * t102;
        let t109 = 1.0 / t108;
        let t111 = f64::exp(-t109 / 4.0);
        let t112 = t111 - 1.0;
        let t115 = t111 - 3.0 / 2.0 - 2.0 * t108 * t112;
        let t118 = 2.0 * t102 * t115 + t103 * t106;
        let t122 = piecewise3(t74, 1.0 / t77 / 36.0 - t81 / 960.0 + t84 / 26880.0 - t87 / 829440.0 + t90 / 28385280.0 - t93 / 0.107347968e10 + t96 / 0.445906944e11 - t99 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t102 * t118);
        let t124 = -param_hyb_coeff_0 * t122 - param_hyb_coeff_1 + 1.0;
        let t125 = t53 * t124;
        let t128 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t125);
        let t129 = rho1 <= dens_threshold;
        let t130 = -t16;
        let t132 = piecewise5(t14, t11, t10, t15, t130 * t7);
        let t133 = 1.0 + t132;
        let t134 = t133 <= zeta_threshold;
        let t135 = pow_1_3(t133);
        let t137 = piecewise3(t134, t22, t135 * t133);
        let t138 = t5 * t137;
        let t139 = param_C * sigma2;
        let t140 = rho1 * rho1;
        let t141 = pow_1_3(rho1);
        let t142 = t141 * t141;
        let t144 = 1.0 / t142 / t140;
        let t146 = sigma2 * sigma2;
        let t147 = param_D * t146;
        let t148 = t140 * t140;
        let t149 = t148 * rho1;
        let t151 = 1.0 / t141 / t149;
        let t153 = t139 * t144 + t147 * t151 + 1.0;
        let t156 = param_B * (1.0 - 1.0 / t153);
        let t157 = param_E * sigma2;
        let t159 = t157 * t144 + 1.0;
        let t161 = 1.0 - 1.0 / t159;
        let t163 = t156 * t161 + param_A;
        let t164 = t27 * t163;
        let t167 = t55 * t60 / t163;
        let t168 = f64::sqrt(t167);
        let t170 = param_hyb_omega_0 / t168;
        let t171 = t133 * t6;
        let t172 = pow_1_3(t171);
        let t173 = 1.0 / t172;
        let t174 = t67 * t173;
        let t176 = t170 * t174 / 2.0;
        let t177 = 0.135e1 <= t176;
        let t178 = 0.135e1 < t176;
        let t179 = piecewise3(t178, t176, 0.135e1);
        let t180 = t179 * t179;
        let t183 = t180 * t180;
        let t184 = 1.0 / t183;
        let t186 = t183 * t180;
        let t187 = 1.0 / t186;
        let t189 = t183 * t183;
        let t190 = 1.0 / t189;
        let t193 = 1.0 / t189 / t180;
        let t196 = 1.0 / t189 / t183;
        let t199 = 1.0 / t189 / t186;
        let t201 = t189 * t189;
        let t202 = 1.0 / t201;
        let t205 = piecewise3(t178, 0.135e1, t176);
        let t206 = 1.0 / t205;
        let t208 = erf_approx(t206 / 2.0);
        let t210 = t205 * t205;
        let t211 = 1.0 / t210;
        let t213 = f64::exp(-t211 / 4.0);
        let t214 = t213 - 1.0;
        let t217 = t213 - 3.0 / 2.0 - 2.0 * t210 * t214;
        let t220 = t103 * t208 + 2.0 * t205 * t217;
        let t224 = piecewise3(t177, 1.0 / t180 / 36.0 - t184 / 960.0 + t187 / 26880.0 - t190 / 829440.0 + t193 / 28385280.0 - t196 / 0.107347968e10 + t199 / 0.445906944e11 - t202 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t205 * t220);
        let t226 = -param_hyb_coeff_0 * t224 - param_hyb_coeff_1 + 1.0;
        let t227 = t164 * t226;
        let t230 = piecewise3(t129, 0.0, -3.0 / 8.0 * t138 * t227);
        let tzk0 = t128 + t230;
        zk[ip] += tzk0;
    }
}

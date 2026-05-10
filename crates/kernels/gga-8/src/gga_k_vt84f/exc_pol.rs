//! GGA_K_VT84F exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 130 shared lines across all orders.
//! Delta: 130 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_k_vt84f_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_alpha: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        // --- shared preamble (130 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = t32 * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t37 = t33 / t35;
        let t38 = f64::sqrt(sigma0);
        let t39 = pow_1_3(rho0);
        let t41 = 1.0 / t39 / rho0;
        let t44 = t37 * t38 * t41 / 12.0;
        let t45 = f64::sqrt(f64::EPSILON);
        let t46 = t44 <= t45;
        let t48 = (-param_mu + param_alpha + 5.0 / 3.0) * t32;
        let t49 = t35 * t35;
        let t50 = 1.0 / t49;
        let t51 = t50 * sigma0;
        let t52 = rho0 * rho0;
        let t53 = t39 * t39;
        let t55 = 1.0 / t53 / t52;
        let t59 = param_mu * param_alpha;
        let t60 = param_mu * param_mu;
        let t62 = (t59 + t60 - param_alpha) * t33;
        let t64 = 1.0 / t35 / t34;
        let t65 = sigma0 * sigma0;
        let t66 = t64 * t65;
        let t67 = t52 * t52;
        let t68 = t67 * rho0;
        let t70 = 1.0 / t39 / t68;
        let t74 = param_alpha * param_alpha;
        let t76 = param_mu * t74 / 2.0;
        let t79 = t74 / 2.0;
        let t81 = t34 * t34;
        let t83 = (-t76 - (t59 + t60) * param_mu - t79) / t81;
        let t84 = t65 * sigma0;
        let t85 = t67 * t67;
        let t86 = 1.0 / t85;
        let t90 = t74 * param_alpha;
        let t94 = t60 * param_mu;
        let t98 = (param_mu * t90 / 6.0 - (-param_alpha * t60 - t76 - t94) * param_mu + t79) * t32;
        let t100 = 1.0 / t49 / t81;
        let t101 = t65 * t65;
        let t102 = t100 * t101;
        let t103 = t85 * t52;
        let t105 = 1.0 / t53 / t103;
        let t110 = t45 < t44;
        let t111 = piecewise3(t110, t44, t45);
        let t112 = t111 * t111;
        let t113 = param_mu * t112;
        let t114 = param_alpha * t112;
        let t115 = f64::exp(-t114);
        let t116 = 1.0 + t113;
        let t117 = 1.0 / t116;
        let t118 = t115 * t117;
        let t120 = t112 * t112;
        let t122 = f64::exp(-param_alpha * t120);
        let t123 = 1.0 - t122;
        let t124 = 1.0 / t112;
        let t125 = t124 - 1.0;
        let t129 = piecewise3(t46, 1.0 + t48 * t51 * t55 / 24.0 + t62 * t66 * t70 / 576.0 + t83 * t84 * t86 / 2304.0 + t98 * t102 * t105 / 55296.0, 1.0 - t113 * t118 + t123 * t125 + 5.0 / 3.0 * t112);
        let t133 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t129);
        let t134 = rho1 <= dens_threshold;
        let t135 = -t17;
        let t137 = piecewise5(t15, t12, t11, t16, t135 * t8);
        let t138 = 1.0 + t137;
        let t139 = t138 <= zeta_threshold;
        let t140 = pow_1_3(t138);
        let t141 = t140 * t140;
        let t143 = piecewise3(t139, t24, t141 * t138);
        let t144 = t143 * t30;
        let t145 = f64::sqrt(sigma2);
        let t146 = pow_1_3(rho1);
        let t148 = 1.0 / t146 / rho1;
        let t151 = t37 * t145 * t148 / 12.0;
        let t152 = t151 <= t45;
        let t153 = t50 * sigma2;
        let t154 = rho1 * rho1;
        let t155 = t146 * t146;
        let t157 = 1.0 / t155 / t154;
        let t161 = sigma2 * sigma2;
        let t162 = t64 * t161;
        let t163 = t154 * t154;
        let t164 = t163 * rho1;
        let t166 = 1.0 / t146 / t164;
        let t170 = t161 * sigma2;
        let t171 = t163 * t163;
        let t172 = 1.0 / t171;
        let t176 = t161 * t161;
        let t177 = t100 * t176;
        let t178 = t171 * t154;
        let t180 = 1.0 / t155 / t178;
        let t185 = t45 < t151;
        let t186 = piecewise3(t185, t151, t45);
        let t187 = t186 * t186;
        let t188 = param_mu * t187;
        let t189 = param_alpha * t187;
        let t190 = f64::exp(-t189);
        let t191 = 1.0 + t188;
        let t192 = 1.0 / t191;
        let t193 = t190 * t192;
        let t195 = t187 * t187;
        let t197 = f64::exp(-param_alpha * t195);
        let t198 = 1.0 - t197;
        let t199 = 1.0 / t187;
        let t200 = t199 - 1.0;
        let t204 = piecewise3(t152, 1.0 + t48 * t153 * t157 / 24.0 + t62 * t162 * t166 / 576.0 + t83 * t170 * t172 / 2304.0 + t98 * t177 * t180 / 55296.0, 1.0 - t188 * t193 + t198 * t200 + 5.0 / 3.0 * t187);
        let t208 = piecewise3(t134, 0.0, 3.0 / 20.0 * t6 * t144 * t204);
        let tzk0 = t133 + t208;
        zk[ip] += tzk0;
    }
}

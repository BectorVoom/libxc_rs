//! GGA_X_ITYH_PBE exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh_pbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ityh_pbe_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_mu: f64,
    param_kappa: f64,
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
        let t28 = t2 * t2;
        let t29 = M_PI * t28;
        let t30 = 1.0 / M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = M_CBRT4;
        let t34 = t32 * t33;
        let t35 = M_CBRT6;
        let t36 = param_mu * t35;
        let t37 = M_PI * M_PI;
        let t38 = pow_1_3(t37);
        let t39 = t38 * t38;
        let t40 = 1.0 / t39;
        let t41 = t40 * sigma0;
        let t42 = rho0 * rho0;
        let t43 = pow_1_3(rho0);
        let t44 = t43 * t43;
        let t46 = 1.0 / t44 / t42;
        let t50 = param_kappa + t36 * t41 * t46 / 24.0;
        let t55 = 1.0 + param_kappa * (1.0 - param_kappa / t50);
        let t58 = t29 * t34 / t55;
        let t59 = rmath::sqrt(t58);
        let t61 = param_hyb_omega_0 / t59;
        let t62 = M_CBRT2;
        let t63 = t19 * t6;
        let t64 = pow_1_3(t63);
        let t65 = 1.0 / t64;
        let t66 = t62 * t65;
        let t68 = t61 * t66 / 2.0;
        let t69 = 1.35 <= t68;
        let t70 = 1.35 < t68;
        let t71 = piecewise3(t70, t68, 1.35);
        let t72 = t71 * t71;
        let t75 = t72 * t72;
        let t76 = 1.0 / t75;
        let t78 = t75 * t72;
        let t79 = 1.0 / t78;
        let t81 = t75 * t75;
        let t82 = 1.0 / t81;
        let t85 = 1.0 / t81 / t72;
        let t88 = 1.0 / t81 / t75;
        let t91 = 1.0 / t81 / t78;
        let t93 = t81 * t81;
        let t94 = 1.0 / t93;
        let t97 = piecewise3(t70, 1.35, t68);
        let t98 = rmath::sqrt(M_PI);
        let t99 = 1.0 / t97;
        let t101 = rmath::erf(t99 / 2.0);
        let t103 = t97 * t97;
        let t104 = 1.0 / t103;
        let t106 = rmath::exp(-t104 / 4.0);
        let t107 = t106 - 1.0;
        let t110 = t106 - 3.0 / 2.0 - 2.0 * t103 * t107;
        let t113 = t98 * t101 + 2.0 * t97 * t110;
        let t117 = piecewise3(t69, 1.0 / t72 / 36.0 - t76 / 960.0 + t79 / 26880.0 - t82 / 829440.0 + t85 / 28385280.0 - t88 / 1073479680.0 + t91 / 44590694400.0 - t94 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t97 * t113);
        let t118 = t27 * t117;
        let t119 = t118 * t55;
        let t122 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t119);
        let t123 = rho1 <= dens_threshold;
        let t124 = -t16;
        let t126 = piecewise5(t14, t11, t10, t15, t124 * t7);
        let t127 = 1.0 + t126;
        let t128 = t127 <= zeta_threshold;
        let t129 = pow_1_3(t127);
        let t131 = piecewise3(t128, t22, t129 * t127);
        let t132 = t5 * t131;
        let t133 = t40 * sigma2;
        let t134 = rho1 * rho1;
        let t135 = pow_1_3(rho1);
        let t136 = t135 * t135;
        let t138 = 1.0 / t136 / t134;
        let t142 = param_kappa + t36 * t133 * t138 / 24.0;
        let t147 = 1.0 + param_kappa * (1.0 - param_kappa / t142);
        let t150 = t29 * t34 / t147;
        let t151 = rmath::sqrt(t150);
        let t153 = param_hyb_omega_0 / t151;
        let t154 = t127 * t6;
        let t155 = pow_1_3(t154);
        let t156 = 1.0 / t155;
        let t157 = t62 * t156;
        let t159 = t153 * t157 / 2.0;
        let t160 = 1.35 <= t159;
        let t161 = 1.35 < t159;
        let t162 = piecewise3(t161, t159, 1.35);
        let t163 = t162 * t162;
        let t166 = t163 * t163;
        let t167 = 1.0 / t166;
        let t169 = t166 * t163;
        let t170 = 1.0 / t169;
        let t172 = t166 * t166;
        let t173 = 1.0 / t172;
        let t176 = 1.0 / t172 / t163;
        let t179 = 1.0 / t172 / t166;
        let t182 = 1.0 / t172 / t169;
        let t184 = t172 * t172;
        let t185 = 1.0 / t184;
        let t188 = piecewise3(t161, 1.35, t159);
        let t189 = 1.0 / t188;
        let t191 = rmath::erf(t189 / 2.0);
        let t193 = t188 * t188;
        let t194 = 1.0 / t193;
        let t196 = rmath::exp(-t194 / 4.0);
        let t197 = t196 - 1.0;
        let t200 = t196 - 3.0 / 2.0 - 2.0 * t193 * t197;
        let t203 = 2.0 * t188 * t200 + t98 * t191;
        let t207 = piecewise3(t160, 1.0 / t163 / 36.0 - t167 / 960.0 + t170 / 26880.0 - t173 / 829440.0 + t176 / 28385280.0 - t179 / 1073479680.0 + t182 / 44590694400.0 - t185 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t188 * t203);
        let t208 = t27 * t207;
        let t209 = t208 * t147;
        let t212 = piecewise3(t123, 0.0, -3.0 / 8.0 * t132 * t209);
        let tzk0 = t122 + t212;
        zk[ip] += tzk0;
    }
}

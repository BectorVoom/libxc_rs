//! MGGA_K_RDA exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 116 shared lines across all orders.
//! Delta: 116 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_rda_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_A0: f64,
    param_A1: f64,
    param_A2: f64,
    param_A3: f64,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_c: f64,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        // --- shared preamble (116 lines) ---
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = rho0 + rho1;
        let t9 = 1.0 / t8;
        let t12 = 2.0 * rho0 * t9 <= zeta_threshold;
        let t13 = zeta_threshold - 1.0;
        let t16 = 2.0 * rho1 * t9 <= zeta_threshold;
        let t17 = -t13;
        let t18 = rho0 - rho1;
        let t20 = piecewise5(t12, t13, t16, t17, t18 * t9);
        let t21 = 1.0 + t20;
        let t22 = t21 <= zeta_threshold;
        let t23 = pow_1_3(zeta_threshold);
        let t24 = t23 * t23;
        let t25 = t24 * zeta_threshold;
        let t26 = pow_1_3(t21);
        let t27 = t26 * t26;
        let t29 = piecewise3(t22, t25, t27 * t21);
        let t30 = pow_1_3(t8);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t45 = t38 * sigma0 * t43;
        let t47 = t33 * t33;
        let t49 = 1.0 / t35 / t34;
        let t50 = t47 * t49;
        let t51 = sigma0 * sigma0;
        let t52 = t39 * t39;
        let t53 = t52 * rho0;
        let t55 = 1.0 / t40 / t53;
        let t57 = t50 * t51 * t55;
        let t58 = param_a * t47;
        let t59 = lapl0 * lapl0;
        let t60 = t49 * t59;
        let t61 = t39 * rho0;
        let t63 = 1.0 / t40 / t61;
        let t64 = t60 * t63;
        let t66 = t58 * t64 + t57;
        let t68 = f64::sqrt(t66);
        let t71 = 1.0 + param_beta1 * t68 / 24.0;
        let t72 = t71 * t71;
        let t73 = 1.0 / t72;
        let t76 = param_b * t47;
        let t78 = t76 * t64 + t57;
        let t79 = t78 * t78;
        let t81 = f64::sqrt(t78);
        let t84 = 1.0 + param_beta2 * t81 / 24.0;
        let t85 = t84 * t84;
        let t86 = t85 * t85;
        let t87 = 1.0 / t86;
        let t90 = param_c * t33;
        let t91 = t37 * lapl0;
        let t93 = 1.0 / t41 / rho0;
        let t97 = t90 * t91 * t93 / 24.0 + t45 / 24.0;
        let t98 = param_A3 * t97;
        let t100 = param_beta3 * t97 + 1.0;
        let t101 = 1.0 / t100;
        let t103 = 5.0 / 72.0 * t45 + param_A0 + param_A1 * t66 * t73 / 576.0 + param_A2 * t79 * t87 / 331776.0 + t98 * t101;
        let t107 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t32 * t103);
        let t108 = rho1 <= dens_threshold;
        let t109 = -t18;
        let t111 = piecewise5(t16, t13, t12, t17, t109 * t9);
        let t112 = 1.0 + t111;
        let t113 = t112 <= zeta_threshold;
        let t114 = pow_1_3(t112);
        let t115 = t114 * t114;
        let t117 = piecewise3(t113, t25, t115 * t112);
        let t118 = t117 * t31;
        let t119 = rho1 * rho1;
        let t120 = pow_1_3(rho1);
        let t121 = t120 * t120;
        let t123 = 1.0 / t121 / t119;
        let t125 = t38 * sigma2 * t123;
        let t127 = sigma2 * sigma2;
        let t128 = t119 * t119;
        let t129 = t128 * rho1;
        let t131 = 1.0 / t120 / t129;
        let t133 = t50 * t127 * t131;
        let t134 = lapl1 * lapl1;
        let t135 = t49 * t134;
        let t136 = t119 * rho1;
        let t138 = 1.0 / t120 / t136;
        let t139 = t135 * t138;
        let t141 = t58 * t139 + t133;
        let t143 = f64::sqrt(t141);
        let t146 = 1.0 + param_beta1 * t143 / 24.0;
        let t147 = t146 * t146;
        let t148 = 1.0 / t147;
        let t152 = t76 * t139 + t133;
        let t153 = t152 * t152;
        let t155 = f64::sqrt(t152);
        let t158 = 1.0 + param_beta2 * t155 / 24.0;
        let t159 = t158 * t158;
        let t160 = t159 * t159;
        let t161 = 1.0 / t160;
        let t164 = t37 * lapl1;
        let t166 = 1.0 / t121 / rho1;
        let t170 = t90 * t164 * t166 / 24.0 + t125 / 24.0;
        let t171 = param_A3 * t170;
        let t173 = param_beta3 * t170 + 1.0;
        let t174 = 1.0 / t173;
        let t176 = 5.0 / 72.0 * t125 + param_A0 + param_A1 * t141 * t148 / 576.0 + param_A2 * t153 * t161 / 331776.0 + t171 * t174;
        let t180 = piecewise3(t108, 0.0, 3.0 / 20.0 * t7 * t118 * t176);
        let tzk0 = t107 + t180;
        zk[ip] += tzk0;
    }
}

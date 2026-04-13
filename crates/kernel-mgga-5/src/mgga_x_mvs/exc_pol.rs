//! MGGA_X_MVS exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvs.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mvs_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
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
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
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
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = t6 * t26;
        let t28 = pow_1_3(t7);
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t32 = 1.0 / t30 / rho0;
        let t34 = rho0 * rho0;
        let t36 = 1.0 / t30 / t34;
        let t39 = tau0 * t32 - sigma0 * t36 / 8.0;
        let t40 = M_CBRT6;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3(t42);
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t49 = param_k0 * (1.0 - 5.0 / 9.0 * t39 * t40 * t45);
        let t50 = t39 * t39;
        let t52 = t40 * t40;
        let t54 = 1.0 / t43 / t42;
        let t55 = t52 * t54;
        let t58 = 1.0 + 25.0 / 81.0 * param_e1 * t50 * t55;
        let t59 = t58 * t58;
        let t60 = t50 * t50;
        let t62 = t42 * t42;
        let t64 = 1.0 / t44 / t62;
        let t65 = t40 * t64;
        let t68 = t59 + 1250.0 / 2187.0 * param_c1 * t60 * t65;
        let t69 = pow_1_4(t68);
        let t70 = 1.0 / t69;
        let t72 = t49 * t70 + 1.0;
        let t74 = param_b * t52;
        let t75 = sigma0 * sigma0;
        let t76 = t54 * t75;
        let t77 = t34 * t34;
        let t78 = t77 * rho0;
        let t80 = 1.0 / t29 / t78;
        let t84 = 1.0 + t74 * t76 * t80 / 576.0;
        let t85 = f64::powf(t84, 1.0 / 8.0);
        let t86 = 1.0 / t85;
        let t87 = t28 * t72 * t86;
        let t90 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t87);
        let t91 = rho1 <= dens_threshold;
        let t92 = -t17;
        let t94 = piecewise5(t15, t12, t11, t16, t92 * t8);
        let t95 = 1.0 + t94;
        let t96 = t95 <= zeta_threshold;
        let t97 = pow_1_3(t95);
        let t99 = piecewise3(t96, t23, t97 * t95);
        let t100 = t6 * t99;
        let t101 = pow_1_3(rho1);
        let t102 = t101 * t101;
        let t104 = 1.0 / t102 / rho1;
        let t106 = rho1 * rho1;
        let t108 = 1.0 / t102 / t106;
        let t111 = tau1 * t104 - sigma2 * t108 / 8.0;
        let t116 = param_k0 * (1.0 - 5.0 / 9.0 * t111 * t40 * t45);
        let t117 = t111 * t111;
        let t121 = 1.0 + 25.0 / 81.0 * param_e1 * t117 * t55;
        let t122 = t121 * t121;
        let t123 = t117 * t117;
        let t127 = t122 + 1250.0 / 2187.0 * param_c1 * t123 * t65;
        let t128 = pow_1_4(t127);
        let t129 = 1.0 / t128;
        let t131 = t116 * t129 + 1.0;
        let t133 = sigma2 * sigma2;
        let t134 = t54 * t133;
        let t135 = t106 * t106;
        let t136 = t135 * rho1;
        let t138 = 1.0 / t101 / t136;
        let t142 = 1.0 + t74 * t134 * t138 / 576.0;
        let t143 = f64::powf(t142, 1.0 / 8.0);
        let t144 = 1.0 / t143;
        let t145 = t28 * t131 * t144;
        let t148 = piecewise3(t91, 0.0, -3.0 / 8.0 * t100 * t145);
        let tzk0 = t90 + t148;
        zk[ip] += tzk0;
    }
}

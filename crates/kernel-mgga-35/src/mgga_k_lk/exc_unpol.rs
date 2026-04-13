//! MGGA_K_LK exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_lk.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_lk_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t40 = t25 * t25;
        let t42 = 1.0 / t27 / t26;
        let t43 = t40 * t42;
        let t44 = lapl[ip] * lapl[ip];
        let t45 = t44 * t31;
        let t46 = t34 * rho[ip];
        let t48 = 1.0 / t22 / t46;
        let t51 = t43 * t45 * t48 / 2916.0;
        let t52 = t43 * sigma[ip];
        let t53 = t34 * t34;
        let t55 = 1.0 / t22 / t53;
        let t56 = t31 * t55;
        let t57 = t56 * lapl[ip];
        let t59 = t52 * t57 / 2592.0;
        let t60 = sigma[ip] * sigma[ip];
        let t61 = t60 * t31;
        let t62 = t53 * rho[ip];
        let t64 = 1.0 / t22 / t62;
        let t67 = t43 * t61 * t64 / 8748.0;
        let t68 = t43 * t60;
        let t69 = t31 * t64;
        let t70 = 1.0 / param_kappa;
        let t71 = t69 * t70;
        let t76 = 1.0 + (5.0 / 648.0 * t30 * t33 * t36 + t51 - t59 + t67 + 25.0 / 209952.0 * t68 * t71) * t70;
        let t78 = t30 * sigma[ip];
        let t79 = t32 * t36;
        let t80 = t51 - t59 + t67;
        let t81 = t80 * t70;
        let t85 = t26 * t26;
        let t86 = 1.0 / t85;
        let t87 = t60 * sigma[ip];
        let t88 = t86 * t87;
        let t89 = t53 * t53;
        let t90 = 1.0 / t89;
        let t91 = param_kappa * param_kappa;
        let t92 = 1.0 / t91;
        let t93 = t90 * t92;
        let t98 = 1.0 + (5.0 / 324.0 * t78 * t79 * t81 + 125.0 / 0.11337408e8 * t88 * t93) * t70;
        let t102 = 1.0 + param_kappa * (2.0 - 1.0 / t76 - 1.0 / t98);
        let t106 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t21 * t23 * t102);
        let tzk0 = 2.0 * t106;
        zk[ip] += tzk0;
    }
}

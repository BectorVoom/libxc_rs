//! MGGA_K_RDA exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 70 shared lines across all orders.
//! Delta: 70 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_rda_exc_unpol(
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
        // --- shared preamble (70 lines) ---
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
        let t24 = t21 * t23;
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
        let t38 = t30 * t33 * t36;
        let t40 = t25 * t25;
        let t42 = 1.0 / t27 / t26;
        let t43 = t40 * t42;
        let t44 = sigma[ip] * sigma[ip];
        let t45 = t44 * t31;
        let t46 = t34 * t34;
        let t47 = t46 * rho[ip];
        let t49 = 1.0 / t22 / t47;
        let t51 = t43 * t45 * t49;
        let t52 = param_a * t40;
        let t53 = t52 * t42;
        let t54 = lapl[ip] * lapl[ip];
        let t55 = t54 * t31;
        let t56 = t34 * rho[ip];
        let t58 = 1.0 / t22 / t56;
        let t59 = t55 * t58;
        let t62 = 2.0 * t53 * t59 + 2.0 * t51;
        let t64 = f64::sqrt(t62);
        let t67 = 1.0 + param_beta1 * t64 / 24.0;
        let t68 = t67 * t67;
        let t69 = 1.0 / t68;
        let t72 = param_b * t40;
        let t73 = t72 * t42;
        let t76 = 2.0 * t73 * t59 + 2.0 * t51;
        let t77 = t76 * t76;
        let t79 = f64::sqrt(t76);
        let t82 = 1.0 + param_beta2 * t79 / 24.0;
        let t83 = t82 * t82;
        let t84 = t83 * t83;
        let t85 = 1.0 / t84;
        let t88 = param_c * t25;
        let t89 = t88 * t29;
        let t90 = lapl[ip] * t32;
        let t92 = 1.0 / t23 / rho[ip];
        let t96 = t89 * t90 * t92 / 24.0 + t38 / 24.0;
        let t97 = param_A3 * t96;
        let t99 = param_beta3 * t96 + 1.0;
        let t100 = 1.0 / t99;
        let t102 = 5.0 / 72.0 * t38 + param_A0 + param_A1 * t62 * t69 / 576.0 + param_A2 * t77 * t85 / 331776.0 + t97 * t100;
        let t106 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t102);
        let tzk0 = 2.0 * t106;
        zk[ip] += tzk0;
    }
}

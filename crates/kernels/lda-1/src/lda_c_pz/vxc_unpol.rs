//! LDA_C_PZ vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 41 shared lines across all orders.
//! Delta: 17 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_PZ vxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_pz_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_beta1_0: f64,
    param_beta1_1: f64,
    param_beta2_0: f64,
    param_beta2_1: f64,
    param_c_0: f64,
    param_c_1: f64,
    param_d_0: f64,
    param_d_1: f64,
    param_gamma_0: f64,
    param_gamma_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (41 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t1 * t3 * t9;
        let t11 = t10 / 4.0;
        let t12 = 1.0 <= t11;
        let t13 = param_gamma_0;
        let t14 = param_beta1_0;
        let t15 = f64::sqrt(t10);
        let t19 = param_beta2_0 * t1;
        let t20 = t3 * t6;
        let t21 = t20 * t8;
        let t24 = 1.0 + t14 * t15 / 2.0 + t19 * t21 / 4.0;
        let t27 = param_a_0;
        let t28 = f64::ln(t11);
        let t32 = param_c_0 * t1;
        let t33 = t32 * t3;
        let t34 = t9 * t28;
        let t38 = param_d_0 * t1;
        let t42 = piecewise3(t12, t13 / t24, t27 * t28 + param_b_0 + t33 * t34 / 4.0 + t38 * t21 / 4.0);
        let t43 = param_gamma_1;
        let t44 = param_beta1_1;
        let t48 = param_beta2_1 * t1;
        let t51 = 1.0 + t44 * t15 / 2.0 + t48 * t21 / 4.0;
        let t54 = param_a_1;
        let t58 = param_c_1 * t1;
        let t59 = t58 * t3;
        let t63 = param_d_1 * t1;
        let t67 = piecewise3(t12, t43 / t51, t54 * t28 + param_b_1 + t59 * t34 / 4.0 + t63 * t21 / 4.0);
        let t70 = pow_1_3(zeta_threshold);
        let t72 = piecewise3(1.0 <= zeta_threshold, t70 * zeta_threshold, 1.0);
        let t74 = 2.0 * t72 - 2.0;
        let t76 = M_CBRT2;
        let t79 = 1.0 / (2.0 * t76 - 2.0);
        let t80 = (t67 - t42) * t74 * t79;
        let tzk0 = t42 + t80;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (17 lines) ---
        let t81 = t24 * t24;
        let t83 = t13 / t81;
        let t84 = 1.0 / t15;
        let t86 = t14 * t84 * t1;
        let t88 = 1.0 / t7 / rho[ip];
        let t89 = t20 * t88;
        let t93 = -t19 * t89 / 12.0 - t86 * t89 / 12.0;
        let t95 = 1.0 / rho[ip];
        let t99 = t6 * t88 * t28;
        let t107 = piecewise3(t12, -t83 * t93, -t27 * t95 / 3.0 - t33 * t99 / 12.0 - t32 * t89 / 12.0 - t38 * t89 / 12.0);
        let t108 = t51 * t51;
        let t110 = t43 / t108;
        let t112 = t44 * t84 * t1;
        let t116 = -t112 * t89 / 12.0 - t48 * t89 / 12.0;
        let t127 = piecewise3(t12, -t110 * t116, -t54 * t95 / 3.0 - t59 * t99 / 12.0 - t58 * t89 / 12.0 - t63 * t89 / 12.0);
        let t130 = (t127 - t107) * t74 * t79;
        let tvrho0 = t42 + t80 + rho[ip] * (t107 + t130);
        vrho[ip] += tvrho0;
    }
}

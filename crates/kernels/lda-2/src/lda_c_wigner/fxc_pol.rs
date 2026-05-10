//! LDA_C_WIGNER fxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 18 shared lines across all orders.
//! Delta: 27 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_WIGNER fxc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_wigner_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (18 lines) ---
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = t7 * param_a;
        let t9 = M_CBRT3;
        let t10 = 1.0 / M_PI;
        let t11 = pow_1_3(t10);
        let t12 = t9 * t11;
        let t13 = M_CBRT4;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t3);
        let t16 = 1.0 / t15;
        let t20 = param_b + t12 * t14 * t16 / 4.0;
        let t21 = 1.0 / t20;
        let tzk0 = t8 * t21;
        zk[ip] += tzk0;
        // --- vxc delta (14 lines) ---
        let t22 = t1 * t5;
        let t23 = t4 * t3;
        let t24 = 1.0 / t23;
        let t25 = t2 * t24;
        let t27 = -2.0 * t22 + 2.0 * t25;
        let t29 = param_a * t21;
        let t33 = t20 * t20;
        let t34 = 1.0 / t33;
        let t36 = t11 * t14;
        let t37 = t34 * t9 * t36;
        let t39 = t16 * t7 * param_a * t37 / 12.0;
        let tvrho0 = t3 * t27 * t29 + t39 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t41 = 2.0 * t22 + 2.0 * t25;
        let tvrho1 = t3 * t41 * t29 + t39 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        // --- fxc delta (this level) (27 lines) ---
        let t44 = t27 * param_a;
        let t45 = t44 * t21;
        let t47 = t8 * t34;
        let t51 = t12 * t14 / t15 / t3;
        let t53 = t47 * t51 / 18.0;
        let t54 = 2.0 * t5;
        let t56 = 8.0 * t1 * t24;
        let t57 = t4 * t4;
        let t58 = 1.0 / t57;
        let t60 = 6.0 * t2 * t58;
        let t61 = -t54 + t56 - t60;
        let t66 = t16 * t27 * param_a * t37;
        let t68 = t15 * t15;
        let t70 = 1.0 / t68 / t3;
        let t74 = 1.0 / t33 / t20;
        let t75 = t9 * t9;
        let t77 = t11 * t11;
        let t79 = t74 * t75 * t77 * t13;
        let t81 = t70 * t7 * param_a * t79 / 18.0;
        let tv2rho20 = 2.0 * t45 + t53 + t3 * t61 * t29 + t66 / 6.0 + t81;
        v2rho2[ip * 3] += tv2rho20;
        let t82 = t41 * param_a;
        let t83 = t82 * t21;
        let t84 = t54 - t60;
        let t89 = t16 * t41 * param_a * t37;
        let tv2rho21 = t45 + t53 + t83 + t3 * t84 * t29 + t89 / 12.0 + t66 / 12.0 + t81;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t93 = -t54 - t56 - t60;
        let tv2rho22 = 2.0 * t83 + t53 + t3 * t93 * t29 + t89 / 6.0 + t81;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}

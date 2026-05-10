//! LDA_C_GK72 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 25 shared lines across all orders.
//! Delta: 11 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise5};

/// LDA_C_GK72 vxc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_gk72_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (25 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = t12 < 0.7;
        let t14 = f64::ln(t12);
        let t21 = t12 < 10.0;
        let t24 = t1 * t1;
        let t26 = t24 / t3;
        let t30 = f64::sqrt(4.0);
        let t31 = f64::sqrt(t11);
        let t36 = t3 * t3;
        let t38 = t1 / t36;
        let t39 = t8 * t8;
        let t43 = t24 * t36;
        let t45 = t5 / t39;
        let t49 = 1.0 / t31 / t43 / t45 / 4.0;
        let tzk0 = piecewise5(t13, 0.0311 * t14 - 0.048 + 0.00225 * t4 * t10 * t14 - 0.00425 * t11, t21, -0.06156 + 0.01898 * t14, 0.146 * t26 * t5 * t8 + 5.3 * t30 / t31 / t11 - 0.49 * t38 * t6 * t39 - 6.4 * t30 * t49);
        zk[ip] += tzk0;
        // --- vxc delta (this level) (11 lines) ---
        let t53 = 1.0 / t7;
        let t56 = 1.0 / t8 / t7;
        let t57 = t6 * t56;
        let t67 = f64::powf(4.0, 1.0 / 6.0);
        let t68 = t67 * t49;
        let t69 = t4 * t56;
        let t77 = 1.0 / t31 / t2 / t53 / 48.0;
        let t78 = t67 * t77;
        let t82 = piecewise5(t13, -0.010366666666666666 * t53 - 0.00075 * t4 * t57 * t14 + 0.0006666666666666666 * t4 * t57, t21, -0.006326666666666667 * t53, 0.048666666666666664 * t26 * t45 + 10.6 * t68 * t69 - 0.32666666666666666 * t38 * t10 - 21.333333333333332 * t78 * t69);
        let tvrho0 = t7 * t82 + tzk0;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}

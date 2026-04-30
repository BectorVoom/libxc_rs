//! LDA_C_GK72 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 24 shared lines across all orders.
//! Delta: 16 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise5};

/// LDA_C_GK72 fxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (24 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = t11 < 0.7;
        let t13 = f64::ln(t11);
        let t20 = t11 < 10.0;
        let t23 = t1 * t1;
        let t25 = t23 / t3;
        let t29 = f64::sqrt(4.0);
        let t30 = f64::sqrt(t10);
        let t35 = t3 * t3;
        let t37 = t1 / t35;
        let t38 = t7 * t7;
        let t42 = t23 * t35;
        let t44 = t5 / t38;
        let t48 = 1.0 / t30 / t42 / t44 / 4.0;
        let tzk0 = piecewise5(t12, 0.0311 * t13 - 0.048 + 0.00225 * t4 * t9 * t13 - 0.00425 * t10, t20, -0.06156 + 0.01898 * t13, 0.146 * t25 * t5 * t7 + 5.3 * t29 / t30 / t10 - 0.49 * t37 * t6 * t38 - 6.4 * t29 * t48);
        zk[ip] += tzk0;
        // --- vxc delta (10 lines) ---
        let t52 = 1.0 / rho[ip];
        let t55 = 1.0 / t7 / rho[ip];
        let t56 = t6 * t55;
        let t66 = f64::powf(4.0, 1.0 / 6.0);
        let t67 = t66 * t48;
        let t68 = t4 * t55;
        let t76 = 1.0 / t30 / t2 / t52 / 48.0;
        let t77 = t66 * t76;
        let t81 = piecewise5(t12, -0.010366666666666666 * t52 - 0.00075 * t4 * t56 * t13 + 0.0006666666666666666 * t4 * t56, t20, -0.006326666666666667 * t52, 0.048666666666666664 * t25 * t44 + 10.6 * t67 * t68 - 0.32666666666666666 * t37 * t9 - 21.333333333333332 * t77 * t68);
        let tvrho0 = rho[ip] * t81 + tzk0;
        vrho[ip] += tvrho0;
        // --- fxc delta (this level) (16 lines) ---
        let t84 = rho[ip] * rho[ip];
        let t85 = 1.0 / t84;
        let t88 = 1.0 / t7 / t84;
        let t89 = t6 * t88;
        let t99 = t5 / t38 / rho[ip];
        let t102 = t66 * t66;
        let t103 = t102 * t102;
        let t104 = t103 * t66;
        let t105 = t104 * t76;
        let t107 = 1.0 / t38 / t84;
        let t108 = t42 * t107;
        let t111 = t4 * t88;
        let t121 = 1.0 / t30 / t1 / t3 / t2 / t56 / 48.0;
        let t122 = t104 * t121;
        let t128 = piecewise5(t12, 0.010366666666666666 * t85 + 0.001 * t4 * t89 * t13 - 0.0006388888888888889 * t4 * t89, t20, 0.006326666666666667 * t85, -0.03244444444444444 * t25 * t99 + 8.833333333333334 * t105 * t108 - 14.133333333333333 * t67 * t111 + 0.10888888888888888 * t37 * t56 - 24.88888888888889 * t122 * t108 + 28.444444444444443 * t77 * t111);
        let tv2rho20 = rho[ip] * t128 + 2.0 * t81;
        v2rho2[ip] += tv2rho20;
    }
}

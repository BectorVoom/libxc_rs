//! LDA_C_GK72 lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 24 shared lines across all orders.
//! Delta: 10 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise5};

/// LDA_C_GK72 lxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_gk72_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
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
        // --- fxc delta (16 lines) ---
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
        // --- kxc delta (15 lines) ---
        let t131 = t84 * rho[ip];
        let t132 = 1.0 / t131;
        let t135 = 1.0 / t7 / t131;
        let t136 = t6 * t135;
        let t147 = t29 * t121;
        let t148 = t84 * t84;
        let t149 = 1.0 / t148;
        let t150 = t2 * t149;
        let t154 = 1.0 / t38 / t131;
        let t155 = t42 * t154;
        let t158 = t4 * t135;
        let t168 = 1.0 / t30 / t23 / t35 / t2 / t99 / 192.0;
        let t169 = t29 * t168;
        let t177 = piecewise5(t12, -0.020733333333333333 * t132 - 0.0023333333333333335 * t4 * t136 * t13 + 0.0011574074074074073 * t4 * t136, t20, -0.012653333333333334 * t132, 0.05407407407407407 * t25 * t5 * t107 + 123.66666666666667 * t147 * t150 - 35.333333333333336 * t105 * t155 + 32.977777777777774 * t67 * t158 - 0.1451851851851852 * t37 * t89 - 448.0 * t169 * t150 + 99.55555555555556 * t122 * t155 - 66.37037037037037 * t77 * t158);
        let tv3rho30 = rho[ip] * t177 + 3.0 * t128;
        v3rho3[ip] += tv3rho30;
        // --- lxc delta (this level) (10 lines) ---
        let t182 = 1.0 / t7 / t148;
        let t183 = t6 * t182;
        let t196 = t148 * rho[ip];
        let t200 = 1.0 / t7 / t196 * t1 * t3;
        let t204 = t2 / t196;
        let t209 = t42 / t38 / t148;
        let t212 = t4 * t182;
        let t217 = M_PI * M_PI;
        let t234 = piecewise5(t12, 0.0622 * t149 + 0.0077777777777777776 * t4 * t183 * t13 - 0.003080246913580247 * t4 * t183, t20, 0.03796 * t149, -0.14419753086419754 * t25 * t5 * t154 + 742.0 * t66 * t168 * t2 * t200 - 989.3333333333334 * t147 * t204 + 157.03703703703704 * t105 * t209 - 109.92592592592592 * t67 * t212 + 0.3387654320987654 * t37 * t136 - 1.4259259259259258 * t66 / t30 * t217 / t85 * t2 * t200 + 3584.0 * t169 * t204 - 442.4691358024691 * t122 * t209 + 221.23456790123456 * t77 * t212);
        let tv4rho40 = rho[ip] * t234 + 4.0 * t177;
        v4rho4[ip] += tv4rho40;
    }
}

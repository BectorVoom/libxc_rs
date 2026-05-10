//! LDA_C_HL exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 58 shared lines across all orders.
//! Delta: 58 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_HL exc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_hl_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_hl_c_0: f64,
    param_hl_c_1: f64,
    param_hl_r_0: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (58 lines) ---
        let t1 = param_hl_c_0;
        let t2 = 1.0 / M_PI;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = param_hl_r_0;
        let t7 = t6 * t6;
        let t8 = t7 * t6;
        let t9 = 1.0 / t8;
        let t12 = 1.0 + 3.0 / 4.0 * t5 * t9;
        let t13 = M_CBRT3;
        let t14 = t13 * t13;
        let t15 = pow_1_3(t2);
        let t16 = 1.0 / t15;
        let t17 = t14 * t16;
        let t18 = M_CBRT4;
        let t19 = pow_1_3(t3);
        let t20 = t18 * t19;
        let t24 = 1.0 + t17 * t20 * t6 / 3.0;
        let t25 = f64::ln(t24);
        let t27 = t15 * t15;
        let t28 = t14 * t27;
        let t29 = t19 * t19;
        let t31 = t18 / t29;
        let t32 = 1.0 / t7;
        let t36 = t13 * t15;
        let t37 = t18 * t18;
        let t39 = t37 / t19;
        let t40 = 1.0 / t6;
        let t45 = t1 * (t12 * t25 - t28 * t31 * t32 / 4.0 + t36 * t39 * t40 / 8.0 - 1.0 / 3.0);
        let t46 = rho0 - rho1;
        let t47 = t46 * t4;
        let t48 = 1.0 + t47;
        let t49 = t48 <= zeta_threshold;
        let t50 = pow_1_3(zeta_threshold);
        let t51 = t50 * zeta_threshold;
        let t52 = pow_1_3(t48);
        let t54 = piecewise3(t49, t51, t52 * t48);
        let t55 = 1.0 - t47;
        let t56 = t55 <= zeta_threshold;
        let t57 = pow_1_3(t55);
        let t59 = piecewise3(t56, t51, t57 * t55);
        let t61 = M_CBRT2;
        let t64 = 1.0 / (2.0 * t61 - 2.0);
        let t65 = (t54 + t59 - 2.0) * t64;
        let t66 = param_hl_c_1;
        let t67 = param_hl_r_1;
        let t68 = t67 * t67;
        let t69 = t68 * t67;
        let t70 = 1.0 / t69;
        let t73 = 1.0 + 3.0 / 4.0 * t5 * t70;
        let t77 = 1.0 + t17 * t20 * t67 / 3.0;
        let t78 = f64::ln(t77);
        let t80 = 1.0 / t68;
        let t84 = 1.0 / t67;
        let t90 = -t66 * (t73 * t78 - t28 * t31 * t80 / 4.0 + t36 * t39 * t84 / 8.0 - 1.0 / 3.0) + t45;
        let t91 = t65 * t90;
        let tzk0 = -t45 + t91;
        zk[ip] += tzk0;
    }
}

//! LDA_C_ML1 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 58 shared lines across all orders.
//! Delta: 58 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

/// LDA_C_ML1 exc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_ml1_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    param_fc: f64,
    param_q: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (58 lines) ---
        let t1 = rho0 + rho1;
        let t2 = rho0 - rho1;
        let t3 = 1.0 / t1;
        let t4 = t2 * t3;
        let t5 = f64::abs(t4);
        let t7 = 1.0 - t5 <= zeta_threshold;
        let t8 = t2 * t2;
        let t9 = t1 * t1;
        let t10 = 1.0 / t9;
        let t12 = -t8 * t10 + 1.0;
        let t13 = pow_1_3(t1);
        let t14 = t13 * param_fc;
        let t16 = 1.0 + t4 <= zeta_threshold;
        let t17 = zeta_threshold - 1.0;
        let t19 = 1.0 - t4 <= zeta_threshold;
        let t21 = piecewise5(t16, t17, t19, -t17, t4);
        let t22 = 1.0 + t21;
        let t23 = f64::powf(t22, param_q);
        let t24 = 1.0 - t21;
        let t25 = f64::powf(t24, param_q);
        let t26 = t23 + t25;
        let t27 = t21 * t21;
        let t28 = 1.0 - t27;
        let t29 = pow_1_3(t28);
        let t30 = t26 * t29;
        let t31 = pow_1_3(t22);
        let t32 = pow_1_3(t24);
        let t33 = t31 + t32;
        let t34 = 1.0 / t33;
        let t35 = t30 * t34;
        let t38 = 1.0 + 10.874334072525 * t14 * t35;
        let t41 = 1.0 / t13;
        let t42 = 1.0 / param_fc;
        let t43 = t41 * t42;
        let t44 = 1.0 / t26;
        let t45 = 1.0 / t29;
        let t46 = t44 * t45;
        let t47 = t46 * t33;
        let t48 = t43 * t47;
        let t50 = 1.0 + 0.09195962397381102 * t48;
        let t51 = f64::ln(t50);
        let t52 = t51 * t41;
        let t53 = t52 * t42;
        let t57 = t13 * t13;
        let t58 = 1.0 / t57;
        let t59 = param_fc * param_fc;
        let t60 = 1.0 / t59;
        let t61 = t58 * t60;
        let t62 = t26 * t26;
        let t63 = 1.0 / t62;
        let t64 = t29 * t29;
        let t65 = 1.0 / t64;
        let t66 = t63 * t65;
        let t67 = t33 * t33;
        let t68 = t66 * t67;
        let t71 = -2.763169 / t38 + 0.28144540420067765 * t53 * t47 + 0.2541000285260132 * t48 - 0.049248579417833935 * t61 * t68;
        let t74 = piecewise3(t7, 0.0, t12 * t71 / 4.0);
        let tzk0 = t1 * t74;
        zk[ip] += tzk0;
    }
}

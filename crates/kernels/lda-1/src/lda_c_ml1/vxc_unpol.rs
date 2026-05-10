//! LDA_C_ML1 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 35 shared lines across all orders.
//! Delta: 9 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

/// LDA_C_ML1 vxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_ml1_vxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    param_fc: f64,
    param_q: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (35 lines) ---
        let t1 = 1.0 <= zeta_threshold;
        let t2 = pow_1_3(rho[ip]);
        let t4 = zeta_threshold - 1.0;
        let t6 = piecewise5(t1, t4, t1, -t4, 0.0);
        let t7 = 1.0 + t6;
        let t8 = f64::powf(t7, param_q);
        let t9 = 1.0 - t6;
        let t10 = f64::powf(t9, param_q);
        let t11 = t8 + t10;
        let t12 = t6 * t6;
        let t13 = 1.0 - t12;
        let t14 = pow_1_3(t13);
        let t16 = pow_1_3(t7);
        let t17 = pow_1_3(t9);
        let t18 = t16 + t17;
        let t20 = t11 * t14 / t18;
        let t23 = 1.0 + 10.874334072525 * t2 * param_fc * t20;
        let t26 = 1.0 / t2;
        let t27 = 1.0 / param_fc;
        let t32 = 1.0 / t11 / t14 * t18;
        let t33 = t26 * t27 * t32;
        let t35 = 1.0 + 0.09195962397381102 * t33;
        let t36 = f64::ln(t35);
        let t42 = t2 * t2;
        let t43 = 1.0 / t42;
        let t44 = param_fc * param_fc;
        let t45 = 1.0 / t44;
        let t47 = t11 * t11;
        let t48 = 1.0 / t47;
        let t49 = t14 * t14;
        let t50 = 1.0 / t49;
        let t52 = t18 * t18;
        let t53 = t48 * t50 * t52;
        let t57 = piecewise3(t1, 0.0, -0.69079225 / t23 + 0.07036135105016941 * t36 * t26 * t27 * t32 + 0.0635250071315033 * t33 - 0.012312144854458484 * t43 * t45 * t53);
        let tzk0 = rho[ip] * t57;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (9 lines) ---
        let t59 = rho[ip] * rho[ip];
        let t60 = t23 * t23;
        let t61 = 1.0 / t60;
        let t67 = 1.0 / t42 / rho[ip];
        let t68 = t67 * t45;
        let t72 = t50 * t52 / t35;
        let t76 = 1.0 / t2 / rho[ip];
        let t87 = piecewise3(t1, 0.0, 2.5039685670704026 * t61 * t43 * param_fc * t20 - 0.002156801128287631 * t68 * t48 * t72 - 0.023453783683389805 * t36 * t76 * t27 * t32 - 0.021175002377167768 * t76 * t27 * t32 + 0.008208096569638989 * t68 * t53);
        let tvrho0 = t59 * t87 + 2.0 * tzk0;
        vrho[ip] += tvrho0;
    }
}

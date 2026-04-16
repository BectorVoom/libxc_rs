//! LDA_C_RPA kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 13 shared lines across all orders.
//! Delta: 6 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_RPA kxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (13 lines) ---
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t12 = f64::ln(t10 / 4.0);
        let t13 = 0.0311 * t12;
        let t16 = 0.00225 * t4 * t9 * t12;
        let t17 = 0.00425 * t10;
        let tzk0 = t13 - 0.048 + t16 - t17;
        zk[ip] += tzk0;
        // --- vxc delta (5 lines) ---
        let t18 = 1.0 / rho[ip];
        let t22 = t6 / t7 / rho[ip];
        let t24 = t4 * t22 * t12;
        let t26 = t4 * t22;
        let tvrho0 = t13 - 0.048 + t16 - t17 + rho[ip] * (-0.010366666666666666 * t18 - 0.00075 * t24 + 0.0006666666666666666 * t26);
        vrho[ip] += tvrho0;
        // --- fxc delta (6 lines) ---
        let t33 = rho[ip] * rho[ip];
        let t34 = 1.0 / t33;
        let t38 = t6 / t7 / t33;
        let t40 = t4 * t38 * t12;
        let t42 = t4 * t38;
        let tv2rho20 = -0.020733333333333333 * t18 - 0.0015 * t24 + 0.0013333333333333333 * t26 + rho[ip] * (0.010366666666666666 * t34 + 0.001 * t40 - 0.0006388888888888889 * t42);
        v2rho2[ip] += tv2rho20;
        // --- kxc delta (this level) (6 lines) ---
        let t49 = t33 * rho[ip];
        let t50 = 1.0 / t49;
        let t54 = t6 / t7 / t49;
        let t56 = t4 * t54 * t12;
        let t58 = t4 * t54;
        let tv3rho30 = 0.0311 * t34 + 0.003 * t40 - 0.0019166666666666666 * t42 + rho[ip] * (-0.020733333333333333 * t50 - 0.0023333333333333335 * t56 + 0.0011574074074074073 * t58);
        v3rho3[ip] += tv3rho30;
    }
}

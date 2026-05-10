//! LDA_C_LP96 kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 5 shared lines across all orders.
//! Delta: 4 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_LP96 kxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_c_lp96_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (5 lines) ---
        let t1 = pow_1_3(rho[ip]);
        let t3 = param_C2 / t1;
        let t4 = t1 * t1;
        let t6 = param_C3 / t4;
        let tzk0 = param_C1 + t3 + t6;
        zk[ip] += tzk0;
        // --- vxc delta (3 lines) ---
        let t9 = param_C2 / t1 / rho[ip];
        let t13 = param_C3 / t4 / rho[ip];
        let tvrho0 = param_C1 + t3 + t6 + rho[ip] * (-t9 / 3.0 - 2.0 / 3.0 * t13);
        vrho[ip] += tvrho0;
        // --- fxc delta (4 lines) ---
        let t19 = rho[ip] * rho[ip];
        let t22 = param_C2 / t1 / t19;
        let t26 = param_C3 / t4 / t19;
        let tv2rho20 = -2.0 / 3.0 * t9 - 4.0 / 3.0 * t13 + rho[ip] * (4.0 / 9.0 * t22 + 10.0 / 9.0 * t26);
        v2rho2[ip] += tv2rho20;
        // --- kxc delta (this level) (4 lines) ---
        let t32 = t19 * rho[ip];
        let t35 = param_C2 / t1 / t32;
        let t39 = param_C3 / t4 / t32;
        let tv3rho30 = 4.0 / 3.0 * t22 + 10.0 / 3.0 * t26 + rho[ip] * (-28.0 / 27.0 * t35 - 80.0 / 27.0 * t39);
        v3rho3[ip] += tv3rho30;
    }
}

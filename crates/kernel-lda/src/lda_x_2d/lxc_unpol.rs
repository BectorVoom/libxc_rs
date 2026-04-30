//! LDA_X_2D lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 8 shared lines across all orders.
//! Delta: 2 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_X_2D lxc -- unpolarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_x_2d_lxc_unpol(
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
        // --- shared preamble (8 lines) ---
        let t1 = M_SQRT2;
        let t2 = f64::sqrt(M_PI);
        let t4 = t1 / t2;
        let t6 = f64::sqrt(zeta_threshold);
        let t8 = piecewise3(1.0 <= zeta_threshold, t6 * zeta_threshold, 1.0);
        let t9 = f64::sqrt(rho[ip]);
        let t11 = t4 * t8 * t9;
        let tzk0 = -4.0 / 3.0 * t11;
        zk[ip] += tzk0;
        // --- vxc delta (1 lines) ---
        let tvrho0 = -2.0 * t11;
        vrho[ip] += tvrho0;
        // --- fxc delta (1 lines) ---
        let tv2rho20 = -t4 * t8 / t9;
        v2rho2[ip] += tv2rho20;
        // --- kxc delta (1 lines) ---
        let tv3rho30 = t4 * t8 / t9 / rho[ip] / 2.0;
        v3rho3[ip] += tv3rho30;
        // --- lxc delta (this level) (2 lines) ---
        let t21 = rho[ip] * rho[ip];
        let tv4rho40 = -3.0 / 4.0 * t4 * t8 / t9 / t21;
        v4rho4[ip] += tv4rho40;
    }
}

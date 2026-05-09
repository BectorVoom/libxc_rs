//! LDA_XC_TIH kxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 10 shared lines across all orders.
//! Delta: 12 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;

/// LDA_XC_TIH kxc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_xc_tih_kxc_pol(
    rho: &Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (10 lines) ---
        let t4 = f64::tanh(1.0953 + 0.0334789 * rho0 + 0.0334789 * rho1);
        let t9 = f64::tanh(-0.414661 + 0.152399 * rho0 + 0.152399 * rho1);
        let t14 = f64::tanh(-0.354691 + 0.0390837 * rho0 + 0.0390837 * rho1);
        let t19 = f64::tanh(0.0748531 + 0.136598 * rho0 + 0.136598 * rho1);
        let t24 = f64::tanh(-1.41063 + 0.00496577 * rho0 + 0.00496577 * rho1);
        let t29 = f64::tanh(0.48315 + 4.02905 * rho0 + 4.02905 * rho1);
        let t34 = f64::tanh(-0.420166 + 0.0104352 * rho0 + 0.0104352 * rho1);
        let t39 = f64::tanh(1.47409 + 0.442455 * rho0 + 0.442455 * rho1);
        let tvrho0 = 0.625039 - 1.30351 * t4 - 1.37026 * t9 - 1.29598 * t14 + 1.04305 * t19 - 0.909651 * t24 - 0.991782 * t29 - 0.915745 * t34 - 1.95026 * t39;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        // --- fxc delta (11 lines) ---
        let t41 = t4 * t4;
        let t43 = t9 * t9;
        let t45 = t14 * t14;
        let t47 = t19 * t19;
        let t49 = t24 * t24;
        let t51 = t29 * t29;
        let t53 = t34 * t34;
        let t55 = t39 * t39;
        let tv2rho20 = -5.03355413957527 + 0.043640080939 * t41 + 0.20882625374 * t43 + 0.050651693526 * t45 - 0.1424785439 * t47 + 0.00451711764627 * t49 + 3.9959392671 * t51 + 0.009555982224 * t53 + 0.8629022883 * t55;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        // --- kxc delta (this level) (12 lines) ---
        let t58 = 0.0334789 - 0.0334789 * t41;
        let t62 = 0.152399 - 0.152399 * t43;
        let t66 = 0.0390837 - 0.0390837 * t45;
        let t70 = 0.136598 - 0.136598 * t47;
        let t74 = 0.00496577 - 0.00496577 * t49;
        let t78 = 4.02905 - 4.02905 * t51;
        let t82 = 0.0104352 - 0.0104352 * t53;
        let t86 = 0.442455 - 0.442455 * t55;
        let tv3rho30 = 0.087280161878 * t4 * t58 + 0.41765250748 * t9 * t62 + 0.101303387052 * t14 * t66 - 0.2849570878 * t19 * t70 + 0.00903423529254 * t24 * t74 + 7.9918785342 * t29 * t78 + 0.019111964448 * t34 * t82 + 1.7258045766 * t39 * t86;
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}

//! LDA_X_2D lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_2d.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_2d_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_SQRT2;
        let t2 = rmath::sqrt(M_PI);
        let t4 = t1 / t2;
        let t6 = rmath::sqrt(zeta_threshold);
        let t8 = piecewise3(1.0 <= zeta_threshold, t6 * zeta_threshold, 1.0);
        let t9 = rmath::sqrt(rho[ip]);
        let t11 = t4 * t8 * t9;
        let tzk0 = -4.0 / 3.0 * t11;
        zk[ip] += tzk0;
        let tvrho0 = -2.0 * t11;
        vrho[ip] += tvrho0;
        let tv2rho20 = -t4 * t8 / t9;
        v2rho2[ip] += tv2rho20;
        let tv3rho30 = t4 * t8 / t9 / rho[ip] / 2.0;
        v3rho3[ip] += tv3rho30;
        let t21 = rho[ip] * rho[ip];
        let tv4rho40 = -3.0 / 4.0 * t4 * t8 / t9 / t21;
        v4rho4[ip] += tv4rho40;
    }
}

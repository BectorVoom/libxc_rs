//! LDA_X_2D vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_2d.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};

/// LDA_X_2D vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_x_2d_vxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_SQRT2;
        let t2 = f64::sqrt(M_PI);
        let t4 = t1 / t2;
        let t6 = f64::sqrt(zeta_threshold);
        let t8 = piecewise3(1.0 <= zeta_threshold, t6 * zeta_threshold, 1.0);
        let t9 = f64::sqrt(rho[ip]);
        let t11 = t4 * t8 * t9;
        let tzk0 = -4.0 / 3.0 * t11;
        zk[ip] += tzk0;
        let tvrho0 = -2.0 * t11;
        vrho[ip] += tvrho0;
    }
}

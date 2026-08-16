//! LDA_X vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::powers::{pow_1_3};
use libxc_rkernel_math::piecewise::{piecewise3};

/// LDA_X vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_x_vxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t8 = pow_1_3(zeta_threshold);
        let t10 = piecewise3(1.0 <= zeta_threshold, t8 * zeta_threshold, 1.0);
        let t11 = pow_1_3(rho[ip]);
        let t15 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t10 * t11);
        let t16 = param_alpha * t15;
        let tzk0 = 2.0 * t16;
        zk[ip] += tzk0;
        let t17 = rho[ip] * param_alpha;
        let t18 = t11 * t11;
        let t23 = piecewise3(t2, 0.0, -t6 * t10 / t18 / 8.0);
        let tvrho0 = 2.0 * t17 * t23 + 2.0 * t16;
        vrho[ip] += tvrho0;
    }
}

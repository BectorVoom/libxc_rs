//! LDA_X fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
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
        let t33 = piecewise3(t2, 0.0, t6 * t10 / t18 / rho[ip] / 12.0);
        let tv2rho20 = 2.0 * t17 * t33 + 4.0 * param_alpha * t23;
        v2rho2[ip] += tv2rho20;
    }
}

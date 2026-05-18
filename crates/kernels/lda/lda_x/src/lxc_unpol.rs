//! LDA_X lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_X lxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_x_lxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t8 = pow_1_3::<f64>(zeta_threshold);
        let t10 = piecewise3::<f64>(1.0 <= zeta_threshold, t8 * zeta_threshold, 1.0);
        let t11 = pow_1_3::<f64>(rho[ip]);
        let t15 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t10 * t11);
        let t16 = param_alpha * t15;
        let tzk0 = 2.0 * t16;
        zk[ip] += tzk0;
        let t17 = rho[ip] * param_alpha;
        let t18 = t11 * t11;
        let t23 = piecewise3::<f64>(t2, 0.0, -t6 * t10 / t18 / 8.0);
        let tvrho0 = 2.0 * t17 * t23 + 2.0 * t16;
        vrho[ip] += tvrho0;
        let t33 = piecewise3::<f64>(t2, 0.0, t6 * t10 / t18 / rho[ip] / 12.0);
        let tv2rho20 = 2.0 * t17 * t33 + 4.0 * param_alpha * t23;
        v2rho2[ip] += tv2rho20;
        let t38 = rho[ip] * rho[ip];
        let t44 = piecewise3::<f64>(t2, 0.0, -5.0 / 36.0 * t6 * t10 / t18 / t38);
        let tv3rho30 = 2.0 * t17 * t44 + 6.0 * param_alpha * t33;
        v3rho3[ip] += tv3rho30;
        let t55 = piecewise3::<f64>(t2, 0.0, 10.0 / 27.0 * t6 * t10 / t18 / t38 / rho[ip]);
        let tv4rho40 = 2.0 * t17 * t55 + 8.0 * param_alpha * t44;
        v4rho4[ip] += tv4rho40;
    }
}

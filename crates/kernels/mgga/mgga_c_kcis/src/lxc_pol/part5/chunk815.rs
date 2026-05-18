//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 815/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk815<F: Float>(t6450: F, t6477: F, t1030: F, t1697: F, t1745: F, t278: F, t305: F, t3056: F, t3061: F, t339: F, t4831: F, t4843: F, t6307: F, t6310: F, t6313: F, t6317: F, t6353: F, t6432: F) -> (F, F) {
    let t6478 = t6450 + t6477;
    let t6480 = t3056 + F::new(0.46853067927761790996e-2) * t4831 + F::new(0.93706135855523581992e-2) * t4843 + F::new(0.46853067927761790996e-2) * t3061 * t6307 + F::new(0.93706135855523581992e-2) * t1030 * t6310 - F::new(0.23426533963880895498e-2) * t1030 * t6313 + F::new(0.14055920378328537299e-1) * t305 * t6317 - F::new(0.46853067927761790996e-2) * t305 * t6353 - t6432 * t339 - F::new(2.0) * t1697 * t1745 - t278 * t6478;
    (t6478, t6480)
}

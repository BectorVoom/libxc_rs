//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 815/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk815(t6450: f64, t6477: f64, t1030: f64, t1697: f64, t1745: f64, t278: f64, t305: f64, t3056: f64, t3061: f64, t339: f64, t4831: f64, t4843: f64, t6307: f64, t6310: f64, t6313: f64, t6317: f64, t6353: f64, t6432: f64) -> (f64, f64) {
    let t6478 = t6450 + t6477;
    let t6480 = t3056 + 0.46853067927761790996e-2_f64 * t4831 + 0.93706135855523581992e-2_f64 * t4843 + 0.46853067927761790996e-2_f64 * t3061 * t6307 + 0.93706135855523581992e-2_f64 * t1030 * t6310 - 0.23426533963880895498e-2_f64 * t1030 * t6313 + 0.14055920378328537299e-1_f64 * t305 * t6317 - 0.46853067927761790996e-2_f64 * t305 * t6353 - t6432 * t339 - 2.0_f64 * t1697 * t1745 - t278 * t6478;
    (t6478, t6480)
}

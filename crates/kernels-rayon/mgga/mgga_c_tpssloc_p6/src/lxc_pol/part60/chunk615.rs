//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 615/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk615(t1510: f64, t7101: f64, t235: f64, t7823: f64, t1499: f64, t2051: f64, t226: f64, t7095: f64, t7097: f64, t7522: f64, t7526: f64, t7530: f64, t812: f64) -> (f64, f64, f64) {
    let t7837 = t7101 * t1510;
    let t7839 = t235 * t7823;
    let t7841 = -t7095 - 0.3289868133696452873e-1_f64 * t7522 - t7097 - 0.16449340668482264365e-1_f64 * t7526 + 0.16449340668482264365e-1_f64 * t7530 + t1499 * t2051 - t812 * t7837 + t226 * t7839;
    (t7837, t7839, t7841)
}

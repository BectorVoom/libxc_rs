//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 554/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk554(t515: f64, t7448: f64, t1971: f64, t1970: f64, t1969: f64, t7229: f64) -> (f64, f64, f64) {
    let t7449 = t515 * t7448;
    let t7450 = t1971 * t7449;
    let t7451 = t1970 * t7450;
    let t7453 = t7229 * t1969;
    (t7450, t7451, t7453)
}

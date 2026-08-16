//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 560/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk560(t265: f64, t333: f64, t797: f64, t7596: f64, t851: f64, t854: f64, t305: f64, t830: f64, t22: f64, t3851: f64, t262: f64, t2100: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7617 = t265 * t333;
    let t7618 = t797 * t7617;
    let t7620 = t851 * t7596;
    let t7625 = t854 * t7617;
    let t7627 = t305 * t830;
    let t7633 = t3851 * t22;
    let t7638 = t262 * t7596;
    let t7639 = t2100 * t7638;
    (t7617, t7618, t7620, t7625, t7627, t7633, t7638, t7639)
}

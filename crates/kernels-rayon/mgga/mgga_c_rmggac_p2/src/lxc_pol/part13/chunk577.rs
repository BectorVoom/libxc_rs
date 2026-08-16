//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 577/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk577(t7617: f64, t797: f64, t7596: f64, t851: f64, t3810: f64, t7614: f64, t854: f64, t305: f64, t830: f64, t2100: f64, t7587: f64, t2103: f64, t7591: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7618 = t797 * t7617;
    let t7620 = t851 * t7596;
    let t7622 = t3810 * t7614;
    let t7625 = t854 * t7617;
    let t7627 = t305 * t830;
    let t7629 = t2100 * t7587;
    let t7631 = t2103 * t7591;
    (t7618, t7620, t7622, t7625, t7627, t7629, t7631)
}

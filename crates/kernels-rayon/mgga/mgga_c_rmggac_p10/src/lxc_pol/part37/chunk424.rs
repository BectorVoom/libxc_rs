//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 424/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk424(t797: f64, t8700: f64, t2347: f64, t6444: f64, t793: f64, t8704: f64, t851: f64, t8708: f64, t854: f64, t8712: f64, t1632: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8729 = t797 * t8700;
    let t8731 = t6444 * t2347;
    let t8733 = t793 * t8704;
    let t8735 = t851 * t8708;
    let t8737 = t854 * t8712;
    let t8739 = t797 * t8712;
    let t8741 = t793 * t8708;
    let t8743 = t649 * t1632;
    (t8729, t8731, t8733, t8735, t8737, t8739, t8741, t8743)
}

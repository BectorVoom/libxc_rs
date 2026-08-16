//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 178/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk178(t201: f64, t597: f64, t205: f64, t592: f64, t472: f64, t589: f64, t206: f64, t207: f64) -> (f64, f64, f64, f64) {
    let t598 = t597 * t201;
    let t600 = t592 * t205;
    let t602 = t472 * t589;
    let t605 = 3.0_f64 * t206 * t602 - t207 * t600;
    (t598, t600, t602, t605)
}

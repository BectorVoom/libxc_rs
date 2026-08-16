//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 643/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk643(t1973: f64, t8577: f64, t128: f64, t1528: f64, t118: f64, t2001: f64, t675: f64, t2191: f64, t2286: f64, t1603: f64, t1986: f64, t2289: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8578 = t8577 * t1973;
    let t8580 = t128 * t1528;
    let t8581 = t118 * t8580;
    let t8582 = t2001 * t8581;
    let t8583 = t675 * t8582;
    let t8585 = t2191 * t2286;
    let t8587 = t1986 * t1603;
    let t8588 = t675 * t8587;
    let t8590 = t2191 * t2289;
    (t8578, t8582, t8583, t8585, t8587, t8588, t8590)
}

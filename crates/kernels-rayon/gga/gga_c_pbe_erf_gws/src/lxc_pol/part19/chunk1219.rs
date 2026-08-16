//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1219/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1219(t13952: f64, t2210: f64, t2118: f64, t838: f64, t1176: f64, t2332: f64, t903: f64, t3993: f64, t1180: f64, t6589: f64, t13987: f64, t894: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51682 = t13952 * t2210;
    let t51717 = t2118 * t838;
    let t51818 = t1176 * t2332 * t903;
    let t51819 = t51818 * t3993;
    let t51869 = t1176 * t6589 * t1180;
    let t51877 = t13987 * t894;
    (t51682, t51717, t51818, t51819, t51869, t51877)
}

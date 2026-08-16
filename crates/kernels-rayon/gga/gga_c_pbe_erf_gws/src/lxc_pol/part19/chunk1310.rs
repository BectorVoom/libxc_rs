//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1310/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1310(t11501: f64, t14567: f64, t6608: f64, t11615: f64, t14011: f64, t11957: f64, t14101: f64, t14046: f64, t3820: f64, t11739: f64, t4049: f64, t11506: f64, t4039: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56975 = t6608 * t11501 * t14567;
    let t56978 = t14011 * t11615;
    let t56980 = t14101 * t11957;
    let t56982 = t14046 * t3820;
    let t56984 = t4049 * t11739;
    let t56986 = t4039 * t11506;
    (t56975, t56978, t56980, t56982, t56984, t56986)
}

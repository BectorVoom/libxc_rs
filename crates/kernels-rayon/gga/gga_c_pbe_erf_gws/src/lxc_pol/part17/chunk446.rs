//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 446/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk446(t1692: f64, t1714: f64, t1698: f64, t657: f64, t1702: f64, t1688: f64, t1689: f64, t1694: f64, t1700: f64, t1704: f64, t1709: f64, t1710: f64, t25: f64) -> (f64, f64, f64, f64) {
    let t1715 = t1714 * t1692;
    let t1718 = t657 * t1698;
    let t1721 = t657 * t1702;
    let t1724 = t1688 + 0.23994444444444444444e-1_f64 * t1689 - 0.23994444444444444445e-1_f64 * t1694 + 0.71983333333333333334e-1_f64 * t1700 - 0.35991666666666666667e-1_f64 * t1704 + t1709 + 0.8888888888888888889e-2_f64 * t1710 - 0.22222222222222222222e-2_f64 * t25 * t1715 + 0.13333333333333333333e-1_f64 * t25 * t1718 - 0.66666666666666666667e-2_f64 * t25 * t1721;
    (t1715, t1718, t1721, t1724)
}

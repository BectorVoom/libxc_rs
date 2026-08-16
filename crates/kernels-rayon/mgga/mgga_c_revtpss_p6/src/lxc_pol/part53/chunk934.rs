//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 934/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk934(t27931: f64, t27959: f64, t225: f64, t1904: f64, t7242: f64, t689: f64, t786: f64, t7911: f64, t1364: f64, t1398: f64, t1903: f64, t543: f64) -> (f64, f64, f64, f64, f64) {
    let t27960 = t27931 + t27959;
    let t27961 = t27960 * t225;
    let t27965 = t7242 * t1904;
    let t27966 = t689 * t27965;
    let t27968 = t786 * t7911;
    let t27969 = t27968 * t1364;
    let t27972 = t1903 * t1398 * t543;
    (t27960, t27961, t27966, t27969, t27972)
}

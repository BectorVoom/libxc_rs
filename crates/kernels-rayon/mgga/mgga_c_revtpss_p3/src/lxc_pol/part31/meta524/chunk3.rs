//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1894/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1894(t225: f64, t27960: f64, t1904: f64, t7242: f64, t689: f64, t786: f64, t7911: f64, t1364: f64, t1398: f64, t1903: f64, t543: f64, t25931: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27961 = t27960 * t225;
    let t27965 = t7242 * t1904;
    let t27966 = t689 * t27965;
    let t27968 = t786 * t7911;
    let t27969 = t27968 * t1364;
    let t27972 = t1903 * t1398 * t543;
    let t27973 = t25931 * t27972;
    (t27961, t27965, t27966, t27968, t27969, t27972, t27973)
}

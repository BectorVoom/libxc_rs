//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1646/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1646(t11315: f64, t923: f64, t11156: f64, t2908: f64, t141: f64, t11165: f64, t930: f64, t2912: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11316 = t923 * t11315;
    let t11318 = t2908 * t11156;
    let t11319 = t141 * t11318;
    let t11321 = t930 * t11165;
    let t11322 = t141 * t11321;
    let t11326 = t698 * t2912;
    (t11316, t11318, t11319, t11321, t11322, t11326)
}

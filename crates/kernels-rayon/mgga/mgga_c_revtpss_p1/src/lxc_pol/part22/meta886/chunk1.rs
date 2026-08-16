//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3073/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3073(t2923: f64, t4587: f64, t11384: f64, t1596: f64, t11466: f64, t300: f64, t11452: f64, t4669: f64, t11450: f64, t1621: f64, t11507: f64, t1633: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52219 = t4587 * t2923;
    let t52224 = t1596 * t11384;
    let t52238 = t300 * t11466;
    let t52264 = t4669 * t11452;
    let t52320 = t11450 * t1621;
    let t52370 = t11507 * t1633;
    (t52219, t52224, t52238, t52264, t52320, t52370)
}

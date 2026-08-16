//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2997/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2997(t10115: f64, t1570: f64, t11007: f64, t1579: f64, t252: f64, t2771: f64, t2782: f64, t4322: f64, t9292: f64, t2772: f64, t4321: f64, t689: f64) -> (f64, f64, f64, f64, f64) {
    let t50155 = t10115 * t1570;
    let t50161 = t11007 * t1579;
    let t50164 = t2782 * t252 * t50161 * t2771;
    let t50166 = t9292 * t4322;
    let t50169 = t689 * t4321 * t2772;
    (t50155, t50161, t50164, t50166, t50169)
}

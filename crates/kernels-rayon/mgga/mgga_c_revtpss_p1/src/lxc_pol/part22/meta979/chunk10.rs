//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3300/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3300(t18657: f64, t212: f64, t689: f64, t780: f64, t252: f64, t2769: f64, t2782: f64, t6071: f64, t886: f64, t4500: f64, t51421: f64, t14495: f64, t14567: f64) -> (f64, f64, f64, f64) {
    let t62549 = t689 * t212 * t18657 * t780;
    let t62572 = t2782 * t252 * t2769 * t6071 * t886;
    let t62577 = t51421 * t4500;
    let t62583 = t2782 * t14567 * t14495;
    (t62549, t62572, t62577, t62583)
}

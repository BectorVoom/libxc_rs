//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 728/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk728(t159: f64, t2699: f64, t222: f64, t794: f64, t798: f64) -> (f64, f64, f64) {
    let t2700 = t2699 * t159;
    let t2702 = 35.0_f64 / 432.0_f64 * t2700 * t222;
    let t2703 = t794 * t798;
    (t2700, t2702, t2703)
}

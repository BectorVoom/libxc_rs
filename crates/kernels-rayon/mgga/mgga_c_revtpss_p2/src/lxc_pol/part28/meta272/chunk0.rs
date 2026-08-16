//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1218/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1218(t1470: f64, t603: f64, t1469: f64, t6968: f64, t6971: f64, t72: f64, t1927: f64) -> (f64, f64, f64, f64) {
    let t7709 = t603 * t1470;
    let t7714 = 5.0_f64 / 6.0_f64 * t6968 * t1469 + t6971;
    let t7715 = t7714 * t72;
    let t7716 = t7715 * t1927;
    (t7709, t7714, t7715, t7716)
}

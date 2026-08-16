//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1872/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1872(t23185: f64, t4283: f64, t81914: f64, t25300: f64, t81591: f64, t1484: f64, t6552: f64, t6637: f64, t81658: f64, t25303: f64, t6579: f64, t13456: f64, t1888: f64, t6646: f64) -> (f64, f64, f64, f64, f64) {
    let t87544 = t23185 * t81914 * t4283;
    let t87546 = t81591 * t25300;
    let t87554 = t6552 * t6637 * t81658 * t1484;
    let t87565 = t6579 * t25303;
    let t87575 = t1888 * t6646 * t13456;
    (t87544, t87546, t87554, t87565, t87575)
}

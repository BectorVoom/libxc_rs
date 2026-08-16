//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2132/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2132(t232: f64, t46693: f64, t6605: f64, t815: f64, t2628: f64, t58345: f64, t2632: f64, t47262: f64, t22996: f64, t6590: f64, t25130: f64, t828: f64, t9627: f64) -> (f64, f64, f64, f64) {
    let t87495 = t6605 * t815 * t46693 * t232;
    let t87498 = t6605 * t2628 * t58345;
    let t87502 = t6605 * t2628 * t47262 * t2632;
    let t87504 = t6590 * t22996;
    let t87507 = t87504 * t25130 * t9627 * t828;
    (t87495, t87498, t87502, t87507)
}

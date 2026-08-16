//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1068/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1068(t12599: f64, t12607: f64, t12611: f64, t12620: f64, t576: f64, t3916: f64, t699: f64, t3914: f64, t972: f64, t12044: f64, t12045: f64, t12046: f64, t12054: f64, t12152: f64, t12154: f64, t12155: f64, t12156: f64, t12158: f64, t12161: f64, t12162: f64, t12192: f64, t12281: f64, t12592: f64, t2469: f64) -> (f64, f64, f64, f64, f64) {
    let t12622 = t12599 + t12607 + t12611 + t12620;
    let t12623 = t576 * t12622;
    let t12624 = t699 * t3916;
    let t12625 = t3914 * t972;
    let t12628 = 2.0_f64 * t12625 * t2469 - t12044 + t12045 + t12046 + t12054 - t12152 - t12154 - t12155 - t12156 + t12158 + t12161 - t12162 + t12192 + t12281 - t12592;
    (t12622, t12623, t12624, t12625, t12628)
}

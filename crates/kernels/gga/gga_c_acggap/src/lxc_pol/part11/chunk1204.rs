//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1204/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1204<F: Float>(t157: F, t309: F, t463: F, t32130: F, t36433: F, t32029: F, t557: F, t1658: F, t406: F, t2934: F, t609: F, t2132: F, t2331: F, t7885: F, t864: F) -> (F, F, F, F, F) {
    let t36495 = t157 * t463 * t309;
    let t36498 = F::new(0.34694512752820797848e1) * t32130 * t36433 * t36495;
    let t36504 = t32029 * t557;
    let t36511 = t1658 * t406 * t157;
    let t36515 = t2934 * t609;
    let t36526 = t7885 * t2132 * t2331 * t864;
    (t36498, t36504, t36511, t36515, t36526)
}

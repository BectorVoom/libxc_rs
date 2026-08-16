//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2684/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2684(t1897: f64, t40317: f64, t10111: f64, t22: f64, t5759: f64, t49146: f64, t543: f64, t2782: f64, t4100: f64, t48475: f64, t47423: f64, t5741: f64) -> (f64, f64, f64, f64, f64) {
    let t49354 = t40317 * t1897;
    let t49361 = t10111 * t5759 * t22;
    let t49376 = t49146 * t543;
    let t49378 = t2782 * t4100 * t49376;
    let t49380 = t48475 * t543;
    let t49382 = t2782 * t4100 * t49380;
    let t49386 = t47423 * t5741;
    (t49354, t49361, t49378, t49382, t49386)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 896/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk896<F: Float>(t1045: F, t6271: F, t3117: F, t373: F, t6258: F, t371: F, t372: F, t3236: F, t5819: F, t1012: F, t1015: F, t5825: F) -> (F, F, F, F, F, F, F) {
    let t6272 = t6271 * t1045;
    let t6273 = t3117 * t6272;
    let t6276 = t373 * t6258;
    let t6278 = t371 * t372 * t6276;
    let t6284 = t3236 * t5819;
    let t6285 = t1012 * t6284;
    let t6288 = t1015 * t5825;
    (t6272, t6273, t6276, t6278, t6284, t6285, t6288)
}

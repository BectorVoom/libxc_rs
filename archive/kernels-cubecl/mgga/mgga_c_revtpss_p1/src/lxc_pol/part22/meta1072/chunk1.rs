//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3843/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3843<F: Float>(t1412: F, t6843: F, t2661: F, t3938: F, t3992: F, t1399: F, t22020: F, t46766: F, t6864: F, t5658: F, t543: F, t4003: F) -> (F, F, F, F, F) {
    let t73920 = t1412 * t6843;
    let t73923 = t2661 * t3992 * t73920 * t3938;
    let t73927 = t2661 * t3992 * t22020 * t1399;
    let t73929 = t46766 * t6864;
    let t73936 = t5658 * t5658;
    let t73937 = t73936 * t543;
    let t73942 = t73936 * t4003;
    (t73923, t73927, t73929, t73937, t73942)
}

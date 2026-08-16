//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1241/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1241<F: Float>(t29658: F, t686: F, t72: F, t7058: F, t7064: F, t27186: F, t99404: F, t98849: F, t29643: F, t93281: F, t93317: F, t18643: F, t92955: F) -> (F, F, F, F, F, F, F) {
    let t105953 = t29658 * t72 * t686;
    let t105954 = t7058 * t105953;
    let t105956 = t7064 * t105953;
    let t105960 = t99404 * t27186;
    let t105962 = t98849 * t27186;
    let t105973 = t29643 * t72 * t686;
    let t105974 = t93281 * t105973;
    let t105976 = t93317 * t105973;
    let t106006 = t92955 * t18643;
    (t105954, t105956, t105960, t105962, t105974, t105976, t106006)
}

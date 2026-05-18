//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 792/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk792<F: Float>(t216: F, t9747: F, t1408: F, t2482: F, t596: F, t212: F, t225: F, t816: F, t2681: F, t820: F, t124: F, t2237: F, t800: F) -> (F, F, F, F, F) {
    let t9748 = t216 * t9747;
    let t9765 = t2482 * t1408 * t596;
    let t9775 = t816 * t596 * t212 * t225;
    let t9779 = t820 * t1408 * t2681;
    let t9784 = t800 * t124 * t2237 * t212;
    (t9748, t9765, t9775, t9779, t9784)
}

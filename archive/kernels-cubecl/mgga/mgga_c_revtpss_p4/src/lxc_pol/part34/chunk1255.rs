//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1255/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1255<F: Float>(t1646: F, t1651: F, t29807: F, t994: F, t1647: F, t7810: F, t1078: F, t1982: F, t3140: F, t6343: F, t29894: F, t3336: F) -> (F, F, F, F, F) {
    let t107532 = t1646 * t1651;
    let t107566 = t994 * t29807;
    let t107629 = t1647 * t7810;
    let t107636 = t1982 * t6343 * t3140 * t1078;
    let t107741 = t29894 * t3336;
    (t107532, t107566, t107629, t107636, t107741)
}

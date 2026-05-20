//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 748/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk748<F: Float>(t243: F, t836: F, t231: F, t2662: F, t2661: F, t240: F, t596: F) -> (F, F, F, F) {
    let t2663 = t243 * t836;
    let t2664 = t2663 * t231;
    let t2665 = t2662 * t2664;
    let t2666 = t2661 * t2665;
    let t2668 = t596 * t240;
    (t2664, t2665, t2666, t2668)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1036/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1036<F: Float>(t372: F, t5277: F, t3362: F, t471: F, t1285: F, t12865: F, t5302: F, t15904: F, t3623: F) -> (F, F, F, F, F) {
    let t17661 = t372 * t5277;
    let t17687 = t471 * t3362;
    let t17693 = t1285 * t12865;
    let t17694 = t372 * t5302;
    let t17708 = t3623 * t15904;
    (t17661, t17687, t17693, t17694, t17708)
}

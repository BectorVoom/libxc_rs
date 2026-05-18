//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 917/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk917<F: Float>(t1832: F, t1300: F, t198: F, t336: F, t3801: F, t6435: F, t6437: F, t6441: F, t6473: F, t6476: F, t6542: F, t6544: F, t6546: F, t6550: F, t6554: F, t6558: F, t6748: F) -> (F, F) {
    let t6752 = t1832 * t1832;
    let t6756 = t1300 * t198 * t336 * t6748 - t198 * t336 * t3801 * t6752 - t6435 + t6437 - t6441 + t6473 + t6476 + t6542 + t6544 - t6546 + t6550 - t6554 - t6558;
    (t6752, t6756)
}

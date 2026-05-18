//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1031/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1031<F: Float>(t9643: F, t9645: F, t1297: F, t3748: F, t2849: F, t3687: F, t524: F) -> (F, F, F, F, F) {
    let t9646 = t9643 * t9645;
    let t9649 = t3748 * t1297;
    let t9650 = t9649 * t9645;
    let t9653 = t3687 * t2849;
    let t9654 = t524 * t9653;
    (t9646, t9649, t9650, t9653, t9654)
}

//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1257/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1257<F: Float>(t15041: F, t524: F, t531: F, t22714: F, t525: F, t3785: F, t9520: F, t1166: F, t9532: F, t1117: F, t7768: F, t2874: F, t2903: F) -> (F, F, F, F, F, F) {
    let t26437 = t524 * t15041 * t531;
    let t26522 = t524 * t525 * t22714;
    let t26525 = t3785 * t9520;
    let t26534 = t1166 * t9532;
    let t26552 = t1117 * t7768;
    let t26560 = t2903 * t2874;
    (t26437, t26522, t26525, t26534, t26552, t26560)
}

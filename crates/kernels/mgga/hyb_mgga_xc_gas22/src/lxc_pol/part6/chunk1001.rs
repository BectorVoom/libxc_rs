//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1001/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1001<F: Float>(t1117: F, t2880: F, t1123: F, t3701: F, t1139: F, t2903: F, t1129: F, t3727: F, t1134: F, t2874: F, t1539: F, t2893: F, t1145: F, t1128: F, t3785: F, t1535: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9765 = t1117 * t2880;
    let t9766 = t3701 * t1123;
    let t9769 = t2903 * t1139;
    let t9770 = t3727 * t1129;
    let t9773 = t1134 * t2874;
    let t9778 = t1539 * t2893;
    let t9779 = t1145 * t9778;
    let t9782 = t3785 * t1128;
    let t9785 = t1535 * t2893;
    (t9765, t9766, t9769, t9770, t9773, t9778, t9779, t9782, t9785)
}

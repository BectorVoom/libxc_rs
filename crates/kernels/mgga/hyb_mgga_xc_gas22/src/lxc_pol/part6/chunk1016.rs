//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1016/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1016<F: Float>(t1819: F, t3819: F, t555: F, t3823: F, t1782: F, t3814: F, t1787: F, t1179: F, t7913: F, t7920: F, t2997: F, t3: F, t1804: F, t3815: F, t6214: F, t125: F, t3916: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10102 = t555 * t1819 * t3819;
    let t10105 = t555 * t1819 * t3823;
    let t10107 = t1782 * t3814;
    let t10111 = t1787 * t3814;
    let t10115 = t7913 * t1179;
    let t10119 = t7920 * t1179;
    let t10123 = t2997 * t3;
    let t10129 = t1804 * t6214 * t3815;
    let t10131 = t3916 * t125;
    (t10102, t10105, t10107, t10111, t10115, t10119, t10123, t10129, t10131)
}

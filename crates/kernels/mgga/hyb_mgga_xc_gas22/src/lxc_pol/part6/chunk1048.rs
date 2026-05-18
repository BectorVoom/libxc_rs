//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1048/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1048<F: Float>(t1890: F, t3832: F, t3814: F, t6025: F, t545: F, t7945: F, t6033: F, t3008: F, t3: F, t3009: F, t3836: F, t1897: F) -> (F, F, F, F, F, F, F, F) {
    let t9870 = t1890 * t3832;
    let t9872 = t6025 * t3814;
    let t9874 = t7945 * t9872 * t545;
    let t9877 = t6033 * t3814;
    let t9879 = t3008 * t9877 * t545;
    let t9883 = t3008 * t3009 * t3;
    let t9886 = t1890 * t3836;
    let t9888 = t1897 * t3814;
    (t9870, t9872, t9874, t9877, t9879, t9883, t9886, t9888)
}

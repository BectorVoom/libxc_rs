//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 942/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk942<F: Float>(t1864: F, t1867: F, t22075: F, t601: F, t592: F, t6326: F, t6322: F, t6319: F, t6316: F, t1796: F, t509: F, t6617: F, t22531: F, t580: F, t587: F, t22120: F, t22598: F, t22601: F) -> (F, F, F, F, F, F, F, F) {
    let t22652 = 0.51947267698127589897e2 * t601 * t1864 * t22075 * t1867;
    let t22655 = 480.0 * t6326 * t592;
    let t22656 = t6322 * t592;
    let t22657 = 960.0 * t22656;
    let t22658 = t6319 * t592;
    let t22659 = 576.0 * t22658;
    let t22660 = t6316 * t592;
    let t22661 = 96.0 * t22660;
    let t22666 = 0.13012297059337829057e0 * t1796 * t509 * t6617;
    let t22675 = 0.58482233974552040708e0 * t601 * t580 * t22531 * t587;
    let t22694 = 0.91080982599109921211e5 * t601 * t22598 * t22120 * t22601;
    (t22652, t22655, t22657, t22659, t22661, t22666, t22675, t22694)
}

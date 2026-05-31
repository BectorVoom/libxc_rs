//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1020/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1020<F: Float>(t22656: F, t592: F, t6319: F, t6316: F, t1796: F, t509: F, t6617: F, t22531: F, t580: F, t587: F, t601: F, t22120: F, t22598: F, t22601: F) -> (F, F, F, F, F, F) {
    let t22657 = F::cast_from(960.0_f64) * t22656;
    let t22658 = t6319 * t592;
    let t22659 = F::cast_from(576.0_f64) * t22658;
    let t22660 = t6316 * t592;
    let t22661 = F::cast_from(96.0_f64) * t22660;
    let t22666 = F::cast_from(0.13012297059337829057e0_f64) * t1796 * t509 * t6617;
    let t22675 = F::cast_from(0.58482233974552040708e0_f64) * t601 * t580 * t22531 * t587;
    let t22694 = F::cast_from(0.91080982599109921211e5_f64) * t601 * t22598 * t22120 * t22601;
    (t22657, t22659, t22661, t22666, t22675, t22694)
}

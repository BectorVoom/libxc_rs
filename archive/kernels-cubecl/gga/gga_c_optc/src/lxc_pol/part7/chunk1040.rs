//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1040/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1040<F: Float>(t592: F, t6326: F, t6322: F, t6319: F, t6316: F, t544: F, t6340: F, t1796: F, t509: F, t6617: F, t1797: F, t1906: F, t4: F) -> (F, F, F, F, F, F, F) {
    let t22655 = F::cast_from(480.0_f64) * t6326 * t592;
    let t22656 = t6322 * t592;
    let t22657 = F::cast_from(960.0_f64) * t22656;
    let t22658 = t6319 * t592;
    let t22659 = F::cast_from(576.0_f64) * t22658;
    let t22660 = t6316 * t592;
    let t22661 = F::cast_from(96.0_f64) * t22660;
    let t22662 = t544 * t6340;
    let t22663 = F::cast_from(48.0_f64) * t22662;
    let t22666 = F::cast_from(0.13012297059337829057e0_f64) * t1796 * t509 * t6617;
    let t22668 = t1906 * t4 * t1797;
    (t22655, t22657, t22659, t22661, t22663, t22666, t22668)
}

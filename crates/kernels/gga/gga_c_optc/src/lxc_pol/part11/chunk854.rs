//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 854/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk854<F: Float>(t3927: F, t4768: F, t3608: F, t17118: F, t8216: F, t4961: F, t8201: F, t3885: F, t3623: F, t4963: F, t16988: F, t2669: F, t2679: F, t17068: F, t914: F, t17056: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17134 = t3927 * t4768;
    let t17135 = t3608 * t17134;
    let t17138 = t17118 * t8216;
    let t17141 = t8201 * t4961;
    let t17142 = t3885 * t17141;
    let t17145 = t3623 * t4963;
    let t17148 = t2669 * t16988;
    let t17149 = t17148 * t2679;
    let t17152 = t914 * t17068;
    let t17155 = t914 * t17056;
    (t17134, t17135, t17138, t17141, t17142, t17145, t17148, t17149, t17152, t17155)
}

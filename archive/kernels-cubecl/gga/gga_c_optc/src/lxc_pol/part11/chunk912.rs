//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 912/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk912<F: Float>(t17148: F, t2679: F, t17068: F, t914: F, t17056: F, t4961: F, t8002: F, t3623: F, t8129: F, t3927: F, t4776: F, t2813: F) -> (F, F, F, F, F, F, F, F) {
    let t17149 = t17148 * t2679;
    let t17152 = t914 * t17068;
    let t17155 = t914 * t17056;
    let t17160 = t8002 * t4961;
    let t17161 = t3623 * t17160;
    let t17164 = t17148 * t8129;
    let t17169 = t4776 * t3927;
    let t17170 = t2813 * t17169;
    (t17149, t17152, t17155, t17160, t17161, t17164, t17169, t17170)
}

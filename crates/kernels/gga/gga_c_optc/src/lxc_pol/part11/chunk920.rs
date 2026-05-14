//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 920/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk920<F: Float>(t17897: F, t914: F, t17666: F, t1162: F, t1179: F, t12864: F, t16008: F, t16035: F, t16037: F, t16055: F, t16071: F, t16073: F, t17714: F, t4444: F, t4450: F, t5337: F, t5364: F) -> (F, F, F) {
    let t18145 = t914 * t17897;
    let t18161 = t914 * t17666;
    let t18171 = 0.4645868436449114021e2 * t16008 + 0.15454509315180013964e0 * t12864 - 0.12475836244235246496e3 * t16035 - 0.60587206808032502059e1 * t16037 + 0.46363527945540041892e0 * t4450 * t5364 + 0.17386322979577515709e0 * t1162 * t18161 + 0.16121825426676543132e0 * t4444 * t5337 + 0.30228422675018518374e-1 * t1179 * t17714 + 0.84999801233490076802e0 * t16055 - 0.10747883617784362088e0 * t16071 + 0.10076140891672839458e-1 * t16073;
    (t18145, t18161, t18171)
}

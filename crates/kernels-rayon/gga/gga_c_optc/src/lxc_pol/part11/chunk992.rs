//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 992/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk992(t17897: f64, t914: f64, t17666: f64, t1162: f64, t1179: f64, t12864: f64, t16008: f64, t16035: f64, t16037: f64, t16055: f64, t16071: f64, t16073: f64, t17714: f64, t4444: f64, t4450: f64, t5337: f64, t5364: f64) -> (f64, f64, f64) {
    let t18145 = t914 * t17897;
    let t18161 = t914 * t17666;
    let t18171 = 0.4645868436449114021e2_f64 * t16008 + 0.15454509315180013964e0_f64 * t12864 - 0.12475836244235246496e3_f64 * t16035 - 0.60587206808032502059e1_f64 * t16037 + 0.46363527945540041892e0_f64 * t4450 * t5364 + 0.17386322979577515709e0_f64 * t1162 * t18161 + 0.16121825426676543132e0_f64 * t4444 * t5337 + 0.30228422675018518374e-1_f64 * t1179 * t17714 + 0.84999801233490076802e0_f64 * t16055 - 0.10747883617784362088e0_f64 * t16071 + 0.10076140891672839458e-1_f64 * t16073;
    (t18145, t18161, t18171)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 516/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk516<F: Float>(t1239: F, t2849: F, t2855: F, t1066: F, t1464: F, t1450: F, t2941: F, t2958: F, t1456: F, t531: F) -> (F, F, F, F, F, F) {
    let t4070 = t2849 * t1239;
    let t4075 = t2855 * t1239;
    let t4087 = t1464 * t1066;
    let t4095 = t2941 * t1450;
    let t4111 = t2958 * t1450;
    let t4117 = t531 * t1456;
    (t4070, t4075, t4087, t4095, t4111, t4117)
}

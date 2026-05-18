//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1152/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1152<F: Float>(t2640: F, t41521: F, t4947: F, t10894: F, t16968: F, t17079: F, t907: F, t17219: F, t2812: F, t8143: F, t17169: F, t17175: F, t2586: F, t953: F) -> (F, F, F, F, F, F) {
    let t51164 = t2640 * t41521 * t4947;
    let t51169 = t2640 * t10894 * t16968;
    let t51189 = t17079 * t907;
    let t51322 = t2812 * t8143 * t17219;
    let t51325 = t2812 * t8143 * t17169;
    let t51349 = t953 * t2586 * t17175;
    (t51164, t51169, t51189, t51322, t51325, t51349)
}

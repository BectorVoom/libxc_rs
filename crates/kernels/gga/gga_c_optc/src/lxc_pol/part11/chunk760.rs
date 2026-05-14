//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 760/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk760<F: Float>(t1036: F, t5165: F, t2367: F, t5232: F, t1220: F, t5202: F, t8749: F, t5148: F, t531: F) -> (F, F, F, F, F) {
    let t14852 = t5165 * t1036;
    let t14863 = t2367 * t5232;
    let t14864 = t1220 * t14863;
    let t14871 = t8749 * t5202;
    let t14881 = t531 * t5148;
    (t14852, t14863, t14864, t14871, t14881)
}

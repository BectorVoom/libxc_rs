//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 274/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk274<F: Float>(t1023: F, t398: F, t393: F, t1049: F, t401: F) -> (F, F, F, F, F, F, F) {
    let t1060 = 0.17123333333333333333e-1 * t1023;
    let t1065 = t398 * t398;
    let t1066 = 1.0 / t1065;
    let t1067 = t393 * t1066;
    let t1069 = 0.516475e0 * t1023;
    let t1072 = 0.104195e0 * t1049;
    let t1075 = 1.0 / t401;
    (t1060, t1065, t1066, t1067, t1069, t1072, t1075)
}

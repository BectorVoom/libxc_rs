//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 912/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk912<F: Float>(t17696: F, t17875: F, t17896: F, t17945: F, t1550: F, t5351: F, t4411: F, t5430: F, t146: F, t17926: F, t455: F, t17697: F, t3104: F, t3133: F, t9171: F, t4477: F, t5096: F) -> (F, F, F, F, F, F, F, F) {
    let t17947 = t17696 + t17875 + t17896 + t17945;
    let t17960 = t5351 * t1550;
    let t17964 = t4411 * t5430;
    let t17969 = t146 * t455 * t17926;
    let t17978 = t3104 * t17697;
    let t17979 = t17978 * t3133;
    let t17982 = t17978 * t9171;
    let t17987 = t4477 * t5096;
    (t17947, t17960, t17964, t17969, t17978, t17979, t17982, t17987)
}

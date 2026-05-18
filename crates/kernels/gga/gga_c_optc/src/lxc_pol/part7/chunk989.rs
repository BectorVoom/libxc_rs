//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 989/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk989<F: Float>(t1028: F, t1123: F, t2269: F, t297: F, t2849: F, t438: F, t2855: F, t1027: F, t3107: F, t2329: F, t302: F, t2434: F, t875: F) -> (F, F, F, F, F, F, F) {
    let t12743 = t1123 * t1028;
    let t14330 = t297 * t2269;
    let t15305 = t438 * t2849;
    let t15654 = t438 * t2855;
    let t17919 = t3107 * t1027;
    let t18485 = t2329 * t302;
    let t18634 = t2434 * t875;
    (t12743, t14330, t15305, t15654, t17919, t18485, t18634)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1207/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1207<F: Float>(t17336: F, t36845: F, t4300: F, t11900: F, t17340: F, t15066: F, t15067: F, t5110: F, t16241: F, t4075: F, t1025: F, t11: F) -> (F, F, F, F, F) {
    let t58328 = t36845 * t4300 * t17336;
    let t58334 = t11900 * t4300 * t17340;
    let t58338 = t15066 * t15067 * t5110;
    let t58346 = t4075 * t16241;
    let t58348 = t11 * t1025 * t58346;
    (t58328, t58334, t58338, t58346, t58348)
}

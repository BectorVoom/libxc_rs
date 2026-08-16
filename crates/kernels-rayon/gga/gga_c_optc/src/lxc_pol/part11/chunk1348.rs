//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1348/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1348(t17336: f64, t36845: f64, t4300: f64, t11900: f64, t17340: f64, t15066: f64, t15067: f64, t5110: f64, t16241: f64, t4075: f64, t1025: f64, t11: f64) -> (f64, f64, f64, f64, f64) {
    let t58328 = t36845 * t4300 * t17336;
    let t58334 = t11900 * t4300 * t17340;
    let t58338 = t15066 * t15067 * t5110;
    let t58346 = t4075 * t16241;
    let t58348 = t11 * t1025 * t58346;
    (t58328, t58334, t58338, t58346, t58348)
}

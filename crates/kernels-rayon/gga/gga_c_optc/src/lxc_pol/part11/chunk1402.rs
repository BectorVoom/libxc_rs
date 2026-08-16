//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1402/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1402(t58265: f64, t58308: f64, t58310: f64, t58315: f64, t58572: f64, t58591: f64, t58629: f64, t58651: f64, t58788: f64, t58797: f64, t58800: f64, t1102: f64, t15582: f64, t45062: f64) -> (f64, f64) {
    let t59083 = -t58265 + t58308 - t58310 - t58315 - t58572 + t58591 - t58629 - t58651 - t58788 + t58797 - t58800;
    let t59086 = 0.61523382126046769581e4_f64 * t1102 * t15582 * t45062;
    (t59083, t59086)
}

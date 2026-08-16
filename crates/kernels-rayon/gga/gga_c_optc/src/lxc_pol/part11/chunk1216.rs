//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1216/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1216(t37438: f64, t22152: f64, t22274: f64, t22277: f64, t22281: f64, t22285: f64, t3308: f64, t37417: f64, t37422: f64, t38910: f64, t4733: f64, t47989: f64, t48017: f64) -> (f64, f64) {
    let t55994 = 72.0_f64 * t37438;
    let t55995 = t22152 + 6.0_f64 * t47989 + 3.0_f64 * t37417 - 28.0_f64 * t37422 + t22274 + t22277 + t22281 + t22285 + 0.31013857721884116596e-1_f64 * t3308 * t38910 * t4733 - 14.0_f64 / 3.0_f64 * t48017 + t55994;
    (t55994, t55995)
}

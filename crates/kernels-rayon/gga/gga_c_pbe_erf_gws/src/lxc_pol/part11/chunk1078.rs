//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1078/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1078(t10938: f64, t1827: f64, t3346: f64, t587: f64, t32260: f64, t3342: f64, t5543: f64, t39870: f64, t41840: f64, t997: f64, t39883: f64, t39886: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47359 = 8.0_f64 / 15.0_f64 * t587 * t1827 * t10938 * t3346;
    let t47363 = 8.0_f64 / 9.0_f64 * t587 * t5543 * t32260 * t3342;
    let t47364 = 16.0_f64 / 15.0_f64 * t39870;
    let t47366 = 16.0_f64 / 15.0_f64 * t41840 * t997;
    let t47368 = 32.0_f64 / 15.0_f64 * t39883;
    let t47369 = 32.0_f64 / 15.0_f64 * t39886;
    (t47359, t47363, t47364, t47366, t47368, t47369)
}

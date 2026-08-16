//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 289/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk289(t24: f64, t429: f64, t321: f64, t457: f64, t146: f64, t440: f64, t284: f64, t427: f64) -> (f64, f64, f64, f64, f64) {
    let t1156 = t24 * t429;
    let t1157 = t321 * t1156;
    let t1159 = 0.28977204965962526182e-1_f64 * t457 * t1157;
    let t1160 = t146 * t440;
    let t1161 = t427 * t284;
    let t1162 = t1160 * t1161;
    (t1156, t1157, t1159, t1160, t1162)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1142/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1142(t47130: f64, t7290: f64, t4820: f64, t7513: f64, t13892: f64, t5676: f64, t12161: f64, t2033: f64, t2365: f64, t2610: f64, t13848: f64, t7416: f64) -> (f64, f64, f64, f64, f64) {
    let t47484 = t7290 * t47130;
    let t47486 = t7513 * t4820 * t47484;
    let t47488 = t5676 * t13892;
    let t47492 = t2033 * t2365 * t2610 * t12161;
    let t47494 = t7416 * t13848;
    (t47484, t47486, t47488, t47492, t47494)
}

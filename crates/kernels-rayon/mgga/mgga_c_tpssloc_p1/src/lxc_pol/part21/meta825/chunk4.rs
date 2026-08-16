//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2904/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2904(t60449: f64, t60465: f64, t60482: f64, t60498: f64, t60513: f64, t60529: f64, t60546: f64, t60562: f64, t893: f64, t913: f64, t41623: f64, t5730: f64) -> (f64, f64) {
    let t60568 = 1.0_f64 * t893 * (t60449 + t60465 + t60482 + t60498 + t60513 + t60529 + t60546 + t60562) * t913;
    let t60570 = 0.16081979498692535067e2_f64 * t41623 * t5730;
    (t60568, t60570)
}

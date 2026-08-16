//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2897/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2897(t41361: f64, t41363: f64, t41369: f64, t51978: f64, t51981: f64, t51984: f64, t51987: f64, t51990: f64, t51995: f64, t52000: f64, t52004: f64, t52035: f64) -> (f64, f64) {
    let t52588 = 0.53560370370370370369e0_f64 * t51978 - 0.10805407407407407407e0_f64 * t51981 + 0.62517e0_f64 * t51984 + 0.20839e0_f64 * t51987 + 0.62517e0_f64 * t51990 + 0.62517000000000000001e0_f64 * t51995 + 0.55570666666666666666e0_f64 * t52000 - 0.187551e1_f64 * t52004 + 0.16068111111111111111e1_f64 * t41361 + 0.13772666666666666666e1_f64 * t41363 - 0.68863333333333333332e0_f64 * t41369;
    let t52597 = 0.13772666666666666666e1_f64 * t52035;
    (t52588, t52597)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1322/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1322(t22502: f64, t370: f64, t376: f64, t26267: f64, t2942: f64, t2950: f64, t8611: f64, t8647: f64, t8673: f64, t8617: f64, t8644: f64, t25: f64, t26287: f64, t2869: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26424 = 1.0_f64 / t376 / t22502 / t370 / 96.0_f64;
    let t26425 = t26424 * t26267;
    let t26428 = t8611 * t2942 * t2950;
    let t26430 = t8647 * t8673;
    let t26433 = t8617 * t2942 * t2950;
    let t26435 = t8644 * t8673;
    let t26443 = t25 * t2869 * t26287;
    (t26425, t26428, t26430, t26433, t26435, t26443)
}

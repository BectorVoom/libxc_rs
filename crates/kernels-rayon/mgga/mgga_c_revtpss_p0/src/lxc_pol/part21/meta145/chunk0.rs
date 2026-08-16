//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 931/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk931(t3363: f64, t3417: f64, t141: f64, t1145: f64, t3368: f64, t3372: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t3392: f64, t3400: f64, t3402: f64, t3408: f64, t3410: f64, t3414: f64, t3415: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3418 = t3417 * t3363;
    let t3419 = t141 * t3418;
    let t3421 = t1145 * t3368;
    let t3422 = t141 * t3421;
    let t3424 = t1145 * t3372;
    let t3425 = t141 * t3424;
    let t3427 = -0.9494625e0_f64 * t3392 + 0.1898925e1_f64 * t3400 + t3402 - 0.19931111111111111111e0_f64 * t3358 - 0.19931111111111111111e0_f64 * t3365 + 0.59793333333333333334e0_f64 * t3370 + 0.29896666666666666667e0_f64 * t3374 + 0.15358125e0_f64 * t3408 + 0.3071625e0_f64 * t3410 + t3414 - 0.10954222222222222222e0_f64 * t3415 - 0.27385555555555555556e-1_f64 * t3419 + 0.16431333333333333333e0_f64 * t3422 + 0.82156666666666666667e-1_f64 * t3425;
    (t3418, t3419, t3421, t3422, t3424, t3425, t3427)
}

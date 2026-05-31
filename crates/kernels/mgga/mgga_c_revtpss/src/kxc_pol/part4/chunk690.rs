//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 690/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk690<F: Float>(t3363: F, t3417: F, t141: F, t1145: F, t3368: F, t3372: F, t3358: F, t3365: F, t3370: F, t3374: F, t3392: F, t3400: F, t3402: F, t3408: F, t3410: F, t3414: F, t3415: F) -> (F, F, F, F, F, F, F) {
    let t3418 = t3417 * t3363;
    let t3419 = t141 * t3418;
    let t3421 = t1145 * t3368;
    let t3422 = t141 * t3421;
    let t3424 = t1145 * t3372;
    let t3425 = t141 * t3424;
    let t3427 = -F::cast_from(0.9494625e0_f64) * t3392 + F::cast_from(0.1898925e1_f64) * t3400 + t3402 - F::cast_from(0.19931111111111111111e0_f64) * t3358 - F::cast_from(0.19931111111111111111e0_f64) * t3365 + F::cast_from(0.59793333333333333334e0_f64) * t3370 + F::cast_from(0.29896666666666666667e0_f64) * t3374 + F::cast_from(0.15358125e0_f64) * t3408 + F::cast_from(0.3071625e0_f64) * t3410 + t3414 - F::cast_from(0.10954222222222222222e0_f64) * t3415 - F::cast_from(0.27385555555555555556e-1_f64) * t3419 + F::cast_from(0.16431333333333333333e0_f64) * t3422 + F::cast_from(0.82156666666666666667e-1_f64) * t3425;
    (t3418, t3419, t3421, t3422, t3424, t3425, t3427)
}

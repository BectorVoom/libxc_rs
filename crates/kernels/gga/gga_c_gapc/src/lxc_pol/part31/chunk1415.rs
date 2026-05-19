//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1415/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1415<F: Float>(t35527: F, t35536: F, t36375: F, t36376: F, t36377: F, t36378: F, t36379: F, t36380: F, t36381: F, t36383: F, t36384: F, t36386: F, t36387: F, t36388: F) -> F {
    let t38548 = t36375 + t36376 - t36377 + t36378 - t36379 + t36380 - t36381 - F::cast_from(0.5431140175846100239e-5_f64) * t35527 - t36383 - t36384 + F::cast_from(0.5431140175846100239e-5_f64) * t35536 + t36386 + t36387 - t36388;
    t38548
}

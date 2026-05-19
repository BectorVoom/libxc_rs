//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 698/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk698<F: Float>(t13535: F, t2508: F, t2558: F, t3641: F, t943: F, t3650: F, t948: F, t3645: F, t11622: F, t2562: F, t883: F, t11608: F, t935: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13537 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t13535;
    let t13542 = t3641 * t2558;
    let t13543 = t943 * t13542;
    let t13544 = F::cast_from(0.32043859292259267849e-3_f64) * t13543;
    let t13545 = t3650 * t948;
    let t13547 = F::cast_from(0.53833683610995569986e-1_f64) * t2508 * t13545;
    let t13548 = t3645 * t2558;
    let t13549 = t943 * t13548;
    let t13550 = F::cast_from(0.32043859292259267849e-3_f64) * t13549;
    let t13552 = t2562 * t883 * t11622;
    let t13553 = t943 * t13552;
    let t13554 = F::cast_from(0.32043859292259267849e-3_f64) * t13553;
    let t13555 = t11608 * t935;
    (t13537, t13542, t13544, t13545, t13547, t13548, t13550, t13552, t13554, t13555)
}

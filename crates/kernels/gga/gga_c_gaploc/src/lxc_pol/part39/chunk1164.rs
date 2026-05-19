//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1164/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1164<F: Float>(t13924: F, t7137: F, t2508: F, t2580: F, t47225: F, t43212: F, t43216: F, t43220: F, t43222: F, t43224: F, t43231: F, t43233: F, t43237: F, t43243: F) -> F {
    let t47720 = t7137 * t13924;
    let t47723 = t2508 * t2580 * t47225;
    let t47725 = t43212 + t43216 + t43220 + t43222 + F::cast_from(0.32043859292259267849e-3_f64) * t43224 + F::cast_from(0.76905262301422242837e-2_f64) * t43231 + F::cast_from(0.15381052460284448567e-1_f64) * t43233 - t43237 - F::cast_from(0.30762104920568897135e-1_f64) * t47720 + F::cast_from(0.15381052460284448567e-1_f64) * t47723 - t43243;
    t47725
}

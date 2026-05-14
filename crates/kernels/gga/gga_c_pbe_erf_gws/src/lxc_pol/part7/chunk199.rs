//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 199/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk199<F: Float>(t137: F, t131: F, t120: F, t133: F, t156: F, t488: F, t491: F, t498: F) -> (F, F, F, F) {
    let t512 = t137 * t137;
    let t513 = 1.0 / t512;
    let t514 = t131 * t513;
    let t517 = 0.28737583333333333333e0 * t133 * t156 * t120;
    let t520 = -t488 - t491 - t517 - 0.1724255e1 * t133 * t498;
    (t512, t513, t514, t520)
}

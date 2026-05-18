//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 722/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk722<F: Float>(t11022: F, t639: F, t3554: F, t582: F, t211: F, t3478: F, t586: F) -> (F, F, F, F) {
    let t11023 = t639 * t11022;
    let t11025 = t582 * t3554;
    let t11026 = t211 * t11025;
    let t11032 = t3478 * t586;
    (t11023, t11025, t11026, t11032)
}

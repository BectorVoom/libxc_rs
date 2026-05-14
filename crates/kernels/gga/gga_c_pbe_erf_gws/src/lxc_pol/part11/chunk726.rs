//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 726/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk726<F: Float>(t12460: F, t5264: F, t12480: F, t606: F, t10823: F, t10825: F, t10827: F, t12683: F, t12686: F, t25: F, t5241: F, t5271: F, t7374: F, t7407: F, t12682: F, t598: F) -> (F, F, F, F) {
    let t12693 = t5264 * t12460;
    let t12696 = t606 * t12480;
    let t12700 = 0.13333333333333333333e-1 * t25 * t12683 - 0.66666666666666666666e-2 * t25 * t12686 - t5241 + 0.35991666666666666666e-1 * t10827 - 0.22222222222222222222e-1 * t7407 + 0.23994444444444444444e-1 * t10823 - 0.71983333333333333333e-1 * t10825 - 0.29629629629629629629e-2 * t25 * t12693 - 0.66666666666666666667e-2 * t25 * t12696 - t5271 - 0.47988888888888888888e-1 * t7374;
    let t12701 = t12682 + t12700;
    let t12702 = t598 * t12701;
    (t12693, t12696, t12701, t12702)
}

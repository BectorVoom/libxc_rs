//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 781/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk781<F: Float>(t12464: F, t1856: F, t12468: F, t12460: F, t5264: F, t12480: F, t606: F, t10823: F, t10825: F, t10827: F, t25: F, t5241: F, t5271: F, t7374: F, t7407: F) -> (F, F, F, F, F) {
    let t12683 = t1856 * t12464;
    let t12686 = t1856 * t12468;
    let t12693 = t5264 * t12460;
    let t12696 = t606 * t12480;
    let t12700 = F::new(0.13333333333333333333e-1) * t25 * t12683 - F::new(0.66666666666666666666e-2) * t25 * t12686 - t5241 + F::new(0.35991666666666666666e-1) * t10827 - F::new(0.22222222222222222222e-1) * t7407 + F::new(0.23994444444444444444e-1) * t10823 - F::new(0.71983333333333333333e-1) * t10825 - F::new(0.29629629629629629629e-2) * t25 * t12693 - F::new(0.66666666666666666667e-2) * t25 * t12696 - t5271 - F::new(0.47988888888888888888e-1) * t7374;
    (t12683, t12686, t12693, t12696, t12700)
}

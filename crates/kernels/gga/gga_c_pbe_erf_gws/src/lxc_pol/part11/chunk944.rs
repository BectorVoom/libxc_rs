//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 944/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk944<F: Float>(t329: F, t340: F, t6593: F, t20692: F, t825: F, t2200: F, t369: F, t2298: F, t332: F, t21637: F, t378: F, t838: F, t931: F) -> (F, F, F, F, F, F) {
    let t21681 = t329 * t6593 * t340;
    let t21764 = t20692 * t825;
    let t21780 = t329 * t2200 * t369;
    let t21807 = t329 * t332 * t2298;
    let t21823 = F::new(455.0) / F::new(243.0) * t329 * t21637 * t378;
    let t21825 = t329 * t838 * t931;
    (t21681, t21764, t21780, t21807, t21823, t21825)
}

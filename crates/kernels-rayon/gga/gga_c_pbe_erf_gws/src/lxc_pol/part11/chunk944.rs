//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 944/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk944(t329: f64, t340: f64, t6593: f64, t20692: f64, t825: f64, t2200: f64, t369: f64, t2298: f64, t332: f64, t21637: f64, t378: f64, t838: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21681 = t329 * t6593 * t340;
    let t21764 = t20692 * t825;
    let t21780 = t329 * t2200 * t369;
    let t21807 = t329 * t332 * t2298;
    let t21823 = 455.0_f64 / 243.0_f64 * t329 * t21637 * t378;
    let t21825 = t329 * t838 * t931;
    (t21681, t21764, t21780, t21807, t21823, t21825)
}

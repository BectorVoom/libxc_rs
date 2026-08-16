//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1236/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1236(t2397: f64, t4474: f64, t21637: f64, t329: f64, t378: f64, t838: f64, t931: f64, t6819: f64, t2365: f64, t6110: f64, t822: f64, t833: f64) -> (f64, f64, f64, f64) {
    let t21819 = t4474 * t2397;
    let t21823 = 455.0_f64 / 243.0_f64 * t329 * t21637 * t378;
    let t21825 = t329 * t838 * t931;
    let t21826 = t21825 * t6819;
    let t21828 = t6110 * t2365;
    let t21830 = t822 * t21828 * t833;
    (t21819, t21823, t21826, t21830)
}

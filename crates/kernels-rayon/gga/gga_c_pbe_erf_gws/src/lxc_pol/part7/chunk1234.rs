//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1234/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1234(t21773: f64, t2362: f64, t822: f64, t2373: f64, t4453: f64, t2200: f64, t329: f64, t369: f64, t2404: f64, t376: f64, t6738: f64, t829: f64, t830: f64) -> (f64, f64, f64, f64) {
    let t21775 = t822 * t21773 * t2362;
    let t21777 = t4453 * t2373;
    let t21780 = t329 * t2200 * t369;
    let t21781 = t21780 * t2404;
    let t21785 = t829 * t830 * t6738 * t376;
    (t21775, t21777, t21781, t21785)
}

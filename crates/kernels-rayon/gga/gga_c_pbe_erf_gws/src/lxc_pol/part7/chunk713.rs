//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 713/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk713(t5786: f64, t5841: f64, t1570: f64, t513: f64, t1576: f64, t510: f64, t512: f64, t131: f64, t1578: f64, t520: f64, t1590: f64, t120: f64, t133: f64, t1365: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5842 = t5786 + t5841;
    let t5844 = t1570 * t513;
    let t5847 = t510 * t1576;
    let t5852 = t512 * t512;
    let t5853 = 1.0_f64 / t5852;
    let t5854 = t131 * t5853;
    let t5855 = t1578 * t520;
    let t5858 = t520 * t1590;
    let t5863 = 0.89405814814814814813e0_f64 * t133 * t1365 * t120;
    (t5842, t5844, t5847, t5852, t5853, t5854, t5855, t5858, t5863)
}

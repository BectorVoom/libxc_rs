//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 670/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk670(t639: f64, t7459: f64, t219: f64, t641: f64, t1639: f64, t1642: f64, t5219: f64, t995: f64, t1697: f64, t5212: f64, t1802: f64, t589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7460 = t639 * t7459;
    let t7483 = t641 * t219;
    let t7490 = t1639 * t219;
    let t7491 = t7490 * t1642;
    let t7495 = t5219 * t995;
    let t7505 = t5212 * t1697;
    let t7514 = t589 * t1802;
    (t7460, t7483, t7491, t7495, t7505, t7514)
}

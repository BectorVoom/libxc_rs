//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 949/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk949(t5373: f64, t663: f64, t2660: f64, t5346: f64, t16563: f64, t7062: f64, t7069: f64, t5038: f64, t5211: f64, t617: f64, t7483: f64, t4892: f64, t610: f64, t7514: f64) -> (f64, f64, f64, f64, f64) {
    let t17608 = 8.0_f64 / 15.0_f64 * t5373 * t663;
    let t17609 = t2660 * t5346;
    let t17610 = 32.0_f64 / 15.0_f64 * t17609;
    let t17613 = 16.0_f64 / 9.0_f64 * t7062 * t7069 * t16563;
    let t17617 = 64.0_f64 / 15.0_f64 * t5211 * t7483 * t617 * t5038;
    let t17621 = 32.0_f64 / 15.0_f64 * t7062 * t7514 * t610 * t4892;
    (t17608, t17610, t17613, t17617, t17621)
}

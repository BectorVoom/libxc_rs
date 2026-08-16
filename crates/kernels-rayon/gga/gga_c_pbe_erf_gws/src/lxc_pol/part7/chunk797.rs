//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 797/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk797(t2255: f64, t2278: f64, t6598: f64, t2129: f64, t2142: f64, t2123: f64, t6183: f64, t2120: f64, t326: f64, t6469: f64, t4394: f64, t6470: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6600 = t2255 * t2278 * t6598;
    let t6603 = t2129 * t2142;
    let t6604 = 7.0_f64 / 96.0_f64 * t6603;
    let t6605 = t6183 * t2123;
    let t6606 = t2120 * t6605;
    let t6607 = 7.0_f64 / 96.0_f64 * t6606;
    let t6608 = t326 * t6469;
    let t6609 = t6470 * t4394;
    (t6600, t6604, t6605, t6607, t6608, t6609)
}

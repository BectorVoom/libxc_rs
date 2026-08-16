//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 659/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk659(t589: f64, t597: f64, t562: f64, t1828: f64, t5218: f64, t1643: f64, t4367: f64, t642: f64, t639: f64, t4967: f64, t606: f64, t4972: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5219 = t589 * t597;
    let t5220 = t5219 * t562;
    let t5221 = t5220 * t1828;
    let t5223 = 16.0_f64 / 15.0_f64 * t5218 * t5221;
    let t5224 = t1643 * t4367;
    let t5225 = t642 * t5224;
    let t5227 = 8.0_f64 / 15.0_f64 * t639 * t5225;
    let t5233 = t606 * t4967;
    let t5236 = t606 * t4972;
    (t5219, t5220, t5221, t5223, t5224, t5225, t5227, t5233, t5236)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 986/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk986(t2409: f64, t831: f64, t8804: f64, t3214: f64, t4414: f64, t2410: f64, t8589: f64, t1164: f64, t2242: f64, t3123: f64, t6180: f64, t6184: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8806 = t2409 * t831 * t8804;
    let t8810 = 7.0_f64 / 72.0_f64 * t4414 * t3214;
    let t8812 = t2409 * t8589 * t2410;
    let t8818 = t2242 * t1164;
    let t8821 = t3123 * t6180 / 96.0_f64;
    let t8823 = 7.0_f64 / 144.0_f64 * t3123 * t6184;
    (t8806, t8810, t8812, t8818, t8821, t8823)
}

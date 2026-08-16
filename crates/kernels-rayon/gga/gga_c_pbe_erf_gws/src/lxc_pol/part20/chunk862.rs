//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 862/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk862(t3205: f64, t329: f64, t838: f64, t3209: f64, t3214: f64, t4414: f64, t1164: f64, t2242: f64, t3123: f64, t6184: f64, t3133: f64, t6183: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8801 = t329 * t838 * t3205;
    let t8803 = 7.0_f64 / 24.0_f64 * t8801 * t3209;
    let t8810 = 7.0_f64 / 72.0_f64 * t4414 * t3214;
    let t8818 = t2242 * t1164;
    let t8823 = 7.0_f64 / 144.0_f64 * t3123 * t6184;
    let t8824 = t6183 * t3133;
    (t8801, t8803, t8810, t8818, t8823, t8824)
}

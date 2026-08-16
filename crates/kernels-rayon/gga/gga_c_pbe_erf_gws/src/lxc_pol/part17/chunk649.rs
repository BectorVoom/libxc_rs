//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 649/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk649(t2138: f64, t3123: f64, t1125: f64, t2142: f64, t1114: f64, t2145: f64, t2150: f64, t1133: f64, t5: f64) -> (f64, f64, f64, f64, f64) {
    let t3125 = t3123 * t2138 / 96.0_f64;
    let t3126 = t1125 * t2142;
    let t3127 = 7.0_f64 / 288.0_f64 * t3126;
    let t3128 = t1114 * t2145;
    let t3130 = t3128 * t2150 / 48.0_f64;
    let t3131 = t5 * t1133;
    (t3125, t3127, t3128, t3130, t3131)
}

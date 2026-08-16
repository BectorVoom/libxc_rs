//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1076/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1076(t17852: f64, t3421: f64, t3454: f64, t587: f64, t1017: f64, t12464: f64, t5543: f64, t22811: f64, t22813: f64, t34395: f64, t47315: f64, t47319: f64, t47323: f64, t47325: f64, t47327: f64, t47331: f64) -> (f64, f64, f64) {
    let t47335 = 16.0_f64 / 9.0_f64 * t587 * t17852 * t3421 * t3454;
    let t47339 = 32.0_f64 / 9.0_f64 * t587 * t5543 * t12464 * t1017;
    let t47340 = 4.0_f64 / 3.0_f64 * t22811 + 0.72933333333333333331e0_f64 * t22813 + 0.19947266666666666666e0_f64 * t34395 + t47315 - t47319 - t47323 - t47325 + t47327 + t47331 + t47335 + t47339;
    (t47335, t47339, t47340)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 344/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk344(t1008: f64, t1012: f64, t1021: f64, t1026: f64, t1035: f64, t1039: f64, t1048: f64, t1049: f64, t231: f64, t585: f64, t638: f64, t674: f64, t681: f64, t683: f64, t999: f64) -> f64 {
    let t1052 = t999 + t1008 + t585 + t1012 - t1021 + t1026 + t1035 + t638 + t1039 - t1048 + 4.0_f64 / 3.0_f64 * t1049 * t231 + t674 + t681 + t683;
    t1052
}

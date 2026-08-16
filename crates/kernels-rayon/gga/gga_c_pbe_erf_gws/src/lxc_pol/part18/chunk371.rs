//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 371/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk371(t1127: f64, t1138: f64, t1142: f64, t1150: f64, t1154: f64, t1158: f64, t882: f64, t902: f64, t914: f64, t927: f64, t929: f64) -> f64 {
    let t1161 = t1127 - t1138 - t882 - t1142 + t902 * t1150 / 1536.0_f64 - t914 * t1154 / 1536.0_f64 - t927 - t929 * t1158 / 768.0_f64;
    t1161
}

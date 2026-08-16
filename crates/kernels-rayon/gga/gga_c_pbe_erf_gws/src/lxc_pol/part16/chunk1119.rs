//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1119/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1119(t1208: f64, t2242: f64, t4090: f64, t4414: f64, t1205: f64, t6781: f64, t829: f64, t830: f64) -> (f64, f64, f64) {
    let t14302 = 35.0_f64 / 432.0_f64 * t2242 * t1208;
    let t14305 = t4414 * t4090;
    let t14309 = t6781 * t1205;
    let t14311 = t829 * t830 * t14309;
    (t14302, t14305, t14311)
}

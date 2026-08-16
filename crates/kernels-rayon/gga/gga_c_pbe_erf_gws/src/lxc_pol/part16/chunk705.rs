//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 705/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk705(t2409: f64, t3067: f64, t4097: f64, t4034: f64, t4046: f64, t4024: f64, t4030: f64, t4036: f64, t4040: f64, t4044: f64, t4050: f64) -> (f64, f64, f64, f64) {
    let t4099 = t2409 * t3067 * t4097;
    let t4104 = 7.0_f64 / 144.0_f64 * t4034;
    let t4108 = 7.0_f64 / 1152.0_f64 * t4046;
    let t4110 = t4024 / 48.0_f64 - t4030 / 48.0_f64 - t4104 - t4036 / 24.0_f64 + t4040 / 384.0_f64 - t4044 / 384.0_f64 - t4108 - t4050 / 192.0_f64;
    (t4099, t4104, t4108, t4110)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 722/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk722(t1208: f64, t840: f64, t1205: f64, t810: f64, t2376: f64, t2409: f64, t1206: f64, t892: f64, t338: f64, t938: f64, t3067: f64, t4034: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4087 = 7.0_f64 / 288.0_f64 * t840 * t1208;
    let t4088 = t1205 * t810;
    let t4090 = t2409 * t2376 * t4088;
    let t4093 = t892 * t1206;
    let t4094 = t338 * t4093;
    let t4097 = t1205 * t938;
    let t4099 = t2409 * t3067 * t4097;
    let t4104 = 7.0_f64 / 144.0_f64 * t4034;
    (t4087, t4088, t4090, t4094, t4097, t4099, t4104)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 992/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk992(t3178: f64, t337: f64, t814: f64, t2147: f64, t2120: f64, t3180: f64, t6253: f64, t3106: f64, t360: f64, t2306: f64, t3074: f64, t2138: f64) -> (f64, f64, f64) {
    let t8873 = t337 * t3178 * t814;
    let t8874 = t2147 * t8873;
    let t8876 = t2120 * t8874 / 48.0_f64;
    let t8878 = t6253 * t3180 / 48.0_f64;
    let t8879 = t3106 * t360;
    let t8880 = t2306 * t8879;
    let t8881 = t3074 * t8880;
    let t8883 = t8881 * t2138 / 48.0_f64;
    (t8876, t8878, t8883)
}

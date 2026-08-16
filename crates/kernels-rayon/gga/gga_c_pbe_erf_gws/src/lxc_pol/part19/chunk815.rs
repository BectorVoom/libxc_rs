//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 815/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk815(t5219: f64, t572: f64, t108: f64, t182: f64, t267: f64, t1764: f64, t1660: f64, t597: f64, t1663: f64, t2647: f64, t723: f64, t2650: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7055 = t5219 * t572;
    let t7061 = t182 * t108;
    let t7062 = t7061 * t267;
    let t7063 = t5219 * t1764;
    let t7068 = t1660 * t597;
    let t7069 = t7068 * t1663;
    let t7074 = 4.0_f64 / 9.0_f64 * t2647 * t723;
    let t7075 = t2650 * t723;
    (t7055, t7062, t7063, t7069, t7074, t7075)
}

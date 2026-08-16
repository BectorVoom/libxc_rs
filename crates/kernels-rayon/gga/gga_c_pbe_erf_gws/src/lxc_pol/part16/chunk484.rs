//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 484/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk484(t43: f64, t50: f64, t310: f64, t311: f64, t1: f64, t305: f64, t152: f64, t6: f64, t279: f64, t837: f64, t1524: f64, t1526: f64, t1529: f64, t1531: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t2057 = 1.0_f64 / t311 / t310;
    let t2059 = t305 * t2057 * t1;
    let t2060 = t152 * t6;
    let t2062 = t2060 * t837 * t279;
    let t2063 = t2059 * t2062;
    let t2064 = 0.63272429661648472106e0_f64 * t2063;
    let t2068 = piecewise3(t44, 0.0_f64, -2.0_f64 / 9.0_f64 * t1524 + 2.0_f64 / 3.0_f64 * t1526);
    let t2072 = piecewise3(t51, 0.0_f64, -2.0_f64 / 9.0_f64 * t1529 + 2.0_f64 / 3.0_f64 * t1531);
    (t2057, t2059, t2060, t2062, t2064, t2068, t2072)
}

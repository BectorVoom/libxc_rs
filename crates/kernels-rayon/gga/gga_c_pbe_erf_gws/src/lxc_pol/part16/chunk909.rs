//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 909/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk909(t2602: f64, t5493: f64, t639: f64, t1406: f64, t2625: f64, t1885: f64, t1820: f64, t2631: f64, t5018: f64, t587: f64, t1017: f64, t1804: f64, t5175: f64) -> (f64, f64, f64, f64) {
    let t7925 = t5493 * t2602;
    let t7927 = 16.0_f64 / 45.0_f64 * t639 * t7925;
    let t7928 = t2625 * t1406;
    let t7929 = t1885 * t7928;
    let t7931 = 4.0_f64 / 15.0_f64 * t1820 * t7929;
    let t7932 = t5018 * t2631;
    let t7934 = 16.0_f64 / 45.0_f64 * t587 * t7932;
    let t7936 = t5175 * t1017 * t1804;
    (t7927, t7931, t7934, t7936)
}

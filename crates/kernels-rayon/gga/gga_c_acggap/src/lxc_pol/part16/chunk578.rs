//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 578/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk578(t484: f64, t712: f64, t2992: f64, t1381: f64, t691: f64, t1378: f64, t75: f64, t288: f64, t682: f64, t1413: f64, t935: f64, t506: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5034 = t712 * t484;
    let t5038 = 4.0_f64 * t2992;
    let t5040 = t1381 * t691;
    let t5042 = t1378 * t75;
    let t5043 = t5042 * t288;
    let t5044 = 0.11696447245269292414e1_f64 * t5043;
    let t5045 = t1381 * t682;
    let t5086 = 0.42874018118069736972e-3_f64 * t935 * t1413;
    let t5087 = t506 * t879;
    (t5034, t5038, t5040, t5044, t5045, t5086, t5087)
}

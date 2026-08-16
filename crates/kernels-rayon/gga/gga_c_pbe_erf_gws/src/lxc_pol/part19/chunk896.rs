//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 896/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk896(t50: f64, t3351: f64, t4767: f64, t1412: f64, t3354: f64, t1351: f64, t2465: f64, t422: f64, t52: f64, t9801: f64, t59: f64, t9992: f64, t85: f64, zeta_threshold: f64) -> (f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t9993 = t4767 * t3351;
    let t9998 = t1412 * t3354;
    let t10004 = piecewise3(t51, 0.0_f64, -8.0_f64 / 27.0_f64 * t9993 * t422 - 16.0_f64 / 9.0_f64 * t2465 * t1351 + 4.0_f64 / 9.0_f64 * t9998 * t422 + 4.0_f64 / 3.0_f64 * t52 * t9801);
    let t10006 = (t9992 + t10004) * t59;
    let t10007 = t10006 * t85;
    (t10006, t10007)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 832/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk832(t50: f64, t1412: f64, t34: f64, t422: f64, t532: f64, t1413: f64, t1416: f64, t2465: f64, t2468: f64, t39: f64, t52: f64, t6948: f64, t59: f64, t6947: f64, zeta_threshold: f64) -> (f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t6951 = t1412 * t34;
    let t6952 = t532 * t422;
    let t6962 = piecewise3(t51, 0.0_f64, -8.0_f64 / 27.0_f64 * t6948 * t1413 - 16.0_f64 / 9.0_f64 * t6951 * t6952 + 4.0_f64 / 9.0_f64 * t2465 * t1416 - 8.0_f64 / 3.0_f64 * t52 * t532 + 8.0_f64 * t2468 * t39);
    let t6964 = (t6947 + t6962) * t59;
    (t6952, t6964)
}

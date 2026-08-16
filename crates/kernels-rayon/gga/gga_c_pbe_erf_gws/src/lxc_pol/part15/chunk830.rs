//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 830/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk830(t43: f64, t1403: f64, t1407: f64, t2457: f64, t2460: f64, t39: f64, t47: f64, t532: f64, t6933: f64, t6936: f64, t6937: f64, t4767: f64, t954: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t6947 = piecewise3(t44, 0.0_f64, -8.0_f64 / 27.0_f64 * t6933 * t1403 + 16.0_f64 / 9.0_f64 * t6936 * t6937 + 4.0_f64 / 9.0_f64 * t2457 * t1407 + 8.0_f64 / 3.0_f64 * t47 * t532 - 8.0_f64 * t2460 * t39);
    let t6948 = t4767 * t954;
    (t6947, t6948)
}

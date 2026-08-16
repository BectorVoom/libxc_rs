//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 406/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk406(t43: f64, t1402: f64, t1403: f64, t1407: f64, t47: f64, t93: f64, t422: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t1411 = piecewise3(t44, 0.0_f64, 4.0_f64 / 9.0_f64 * t1402 * t1403 + 4.0_f64 / 3.0_f64 * t47 * t1407);
    let t1412 = 1.0_f64 / t93;
    let t1413 = t422 * t422;
    (t1411, t1412, t1413)
}

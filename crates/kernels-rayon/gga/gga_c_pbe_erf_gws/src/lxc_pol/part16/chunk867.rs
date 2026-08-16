//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 867/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk867(t5323: f64, t1809: f64, t7257: f64, t639: f64, t1027: f64, t1793: f64, t4927: f64, t2559: f64, t7336: f64, t587: f64, t197: f64, t5293: f64) -> (f64, f64, f64, f64, f64) {
    let t7424 = 4.0_f64 / 45.0_f64 * t5323;
    let t7425 = t1809 * t7257;
    let t7427 = 8.0_f64 / 15.0_f64 * t639 * t7425;
    let t7428 = t1027 * t1793;
    let t7429 = t4927 * t7428;
    let t7431 = 8.0_f64 / 45.0_f64 * t639 * t7429;
    let t7432 = t2559 * t7336;
    let t7434 = 4.0_f64 / 27.0_f64 * t587 * t7432;
    let t7435 = t5293 * t197;
    (t7424, t7427, t7431, t7434, t7435)
}

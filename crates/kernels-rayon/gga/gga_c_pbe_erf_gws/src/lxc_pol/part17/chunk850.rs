//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 850/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk850(t7194: f64, t7195: f64, t1620: f64, t2591: f64, t649: f64, t2705: f64, t661: f64, t639: f64, t1697: f64, t34: f64, t422: f64, t1639: f64, t331: f64) -> (f64, f64, f64, f64, f64) {
    let t7196 = t7194 * t7195;
    let t7198 = 32.0_f64 / 45.0_f64 * t1620 * t7196;
    let t7199 = t2591 * t649;
    let t7200 = t2705 * t661;
    let t7201 = t7199 * t7200;
    let t7203 = 16.0_f64 / 45.0_f64 * t639 * t7201;
    let t7204 = t1697 * t34;
    let t7205 = t7204 * t422;
    let t7206 = t7194 * t7205;
    let t7208 = 32.0_f64 / 45.0_f64 * t639 * t7206;
    let t7209 = t331 * t1639;
    (t7198, t7203, t7205, t7208, t7209)
}

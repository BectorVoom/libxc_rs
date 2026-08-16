//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 627/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk627(t1076: f64, t532: f64, t1342: f64, t1345: f64, t1349: f64, t1360: f64, t1386: f64, t1388: f64, t1389: f64, t145: f64, t169: f64, t242: f64, t2848: f64, t2996: f64, t2998: f64, t3003: f64) -> f64 {
    let t3007 = t532 * t1076;
    let t3011 = -t1342 + 0.53059442957798955452e-1_f64 * t1345 + t1349 + 0.53059442957798955452e-1_f64 * t2996 - 0.31835665774679373271e-1_f64 * t169 * t2998 * t242 - 0.31835665774679373271e-1_f64 * t3003 - 0.31835665774679373271e-1_f64 * t1360 - t1386 + t1388 - 0.1066501354843587606e0_f64 * t1389 - 0.1066501354843587606e0_f64 * t3007 + 0.533250677421793803e-1_f64 * t145 * t2848;
    t3011
}

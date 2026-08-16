//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1126/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1126(t14390: f64, t810: f64, t1192: f64, t8589: f64, t829: f64, t830: f64, t3083: f64, t4002: f64, t1105: f64, t1193: f64, t353: f64, t4386: f64) -> (f64, f64, f64, f64, f64) {
    let t14392 = t14390 * t810;
    let t14395 = t8589 * t1192;
    let t14397 = t829 * t830 * t14395;
    let t14400 = t3083 * t4002;
    let t14402 = t1193 * t1105;
    let t14403 = t353 * t14402;
    let t14404 = t4386 * t14403;
    (t14392, t14397, t14400, t14402, t14404)
}

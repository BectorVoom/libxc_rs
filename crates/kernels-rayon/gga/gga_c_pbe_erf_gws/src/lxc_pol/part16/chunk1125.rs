//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1125/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1125(t14423: f64, t361: f64, t3223: f64, t13917: f64, t1162: f64, t875: f64, t13796: f64, t3989: f64, t2171: f64, t13859: f64, t2409: f64, t9721: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14424 = t361 * t14423;
    let t14425 = t14424 * t3223;
    let t14426 = t13917 * t14425;
    let t14442 = t1162 * t875;
    let t14443 = t13796 * t14442;
    let t14444 = t3989 * t14443;
    let t14455 = t14423 * t2171;
    let t14456 = t13796 * t14455;
    let t14457 = t13859 * t14456;
    let t14463 = t2409 * t9721;
    (t14424, t14425, t14426, t14443, t14444, t14456, t14457, t14463)
}

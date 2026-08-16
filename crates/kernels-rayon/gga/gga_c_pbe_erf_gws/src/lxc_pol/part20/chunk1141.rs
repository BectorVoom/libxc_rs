//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1141/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1141(t1154: f64, t14079: f64, t3172: f64, t4028: f64, t3184: f64, t14101: f64, t3142: f64, t3148: f64, t3279: f64, t4049: f64, t14011: f64, t3232: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14506 = t14079 * t1154;
    let t14508 = t4028 * t3172;
    let t14510 = t4028 * t3184;
    let t14512 = t14101 * t3142;
    let t14514 = t4028 * t3148;
    let t14516 = t4049 * t3279;
    let t14518 = t14011 * t3232;
    (t14506, t14508, t14510, t14512, t14514, t14516, t14518)
}

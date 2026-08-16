//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 902/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk902(t10063: f64, t128: f64, t10: f64, t156: f64, t3656: f64, t496: f64, t3660: f64, t3665: f64, t501: f64, t395: f64, t3668: f64, t481: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10064 = t128 * t10063;
    let t10065 = t10 * t10064;
    let t10068 = t156 * t3656;
    let t10069 = t496 * t10068;
    let t10071 = t156 * t3660;
    let t10072 = t496 * t10071;
    let t10074 = t501 * t3665;
    let t10075 = t10074 * t395;
    let t10077 = t501 * t3668;
    let t10078 = t10077 * t395;
    let t10081 = t3665 * t481;
    (t10065, t10068, t10069, t10071, t10072, t10075, t10078, t10081)
}

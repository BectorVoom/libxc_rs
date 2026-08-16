//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1129/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1129(t14498: f64, t3249: f64, t3299: f64, t4039: f64, t1154: f64, t14079: f64, t3172: f64, t4028: f64, t3184: f64, t14101: f64, t3142: f64, t3148: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14499 = t14498 * t3249;
    let t14502 = t4039 * t3299;
    let t14506 = t14079 * t1154;
    let t14508 = t4028 * t3172;
    let t14510 = t4028 * t3184;
    let t14512 = t14101 * t3142;
    let t14514 = t4028 * t3148;
    (t14499, t14502, t14506, t14508, t14510, t14512, t14514)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1241/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1241(t14452: f64, t9270: f64, t14759: f64, t4414: f64, t14633: f64, t51666: f64, t13780: f64, t14637: f64, t3990: f64, t9213: f64, t13859: f64, t9702: f64) -> (f64, f64, f64, f64, f64) {
    let t53187 = 7.0_f64 / 72.0_f64 * t9270 * t14452;
    let t53189 = 7.0_f64 / 72.0_f64 * t4414 * t14759;
    let t53198 = t51666 * t14633;
    let t53199 = 7.0_f64 / 576.0_f64 * t53198;
    let t53207 = t14637 * t3990 * t13780 * t9213;
    let t53212 = t13859 * t3990 * t13780 * t9702;
    (t53187, t53189, t53199, t53207, t53212)
}

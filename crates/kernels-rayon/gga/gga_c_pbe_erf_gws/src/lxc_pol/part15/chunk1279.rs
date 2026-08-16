//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1279/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1279(t27047: f64, t3067: f64, t4164: f64, t814: f64, t9296: f64, t938: f64, t1112: f64, t361: f64, t51020: f64, t874: f64, t13917: f64, t343: f64, t824: f64) -> (f64, f64, f64, f64) {
    let t53790 = t27047 * t3067 * t4164 * t814;
    let t53795 = t27047 * t9296 * t4164 * t938;
    let t53799 = t361 * t51020 * t1112;
    let t53800 = t938 * t874;
    let t53804 = t13917 * t53799 * t824 * t53800 * t343;
    (t53790, t53795, t53800, t53804)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1279/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1279(t12008: f64, t13917: f64, t13919: f64, t14469: f64, t53688: f64, t3258: f64, t51021: f64, t56246: f64, t814: f64, t376: f64, t3824: f64) -> (f64, f64, f64, f64) {
    let t56265 = t13917 * t13919 * t12008;
    let t56269 = t53688 * t14469;
    let t56276 = t13917 * t51021 * t3258 * t56246 * t814;
    let t56296 = t376 * t3824;
    (t56265, t56269, t56276, t56296)
}

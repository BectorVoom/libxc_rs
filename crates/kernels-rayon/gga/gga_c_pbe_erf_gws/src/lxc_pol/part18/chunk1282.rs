//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1282/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1282(t12008: f64, t13917: f64, t13919: f64, t15353: f64, t9270: f64, t14469: f64, t53688: f64, t3258: f64, t51021: f64, t56246: f64, t814: f64, t1105: f64, t353: f64, t4183: f64, t4386: f64) -> (f64, f64, f64, f64, f64) {
    let t56265 = t13917 * t13919 * t12008;
    let t56267 = t9270 * t15353;
    let t56269 = t53688 * t14469;
    let t56276 = t13917 * t51021 * t3258 * t56246 * t814;
    let t56282 = t4386 * t353 * t4183 * t1105;
    (t56265, t56267, t56269, t56276, t56282)
}

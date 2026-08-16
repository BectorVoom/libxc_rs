//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1087/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1087(t1178: f64, t2418: f64, t371: f64, t1177: f64, t2338: f64, t3975: f64, t3972: f64, t13888: f64, t938: f64, t353: f64, t859: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13903 = t371 * t1178 * t2418;
    let t13904 = t1177 * t13903;
    let t13906 = t3975 * t2338;
    let t13907 = t3972 * t13906;
    let t13909 = t13888 * t938;
    let t13910 = t353 * t13909;
    let t13911 = t859 * t13910;
    (t13903, t13904, t13906, t13907, t13909, t13910, t13911)
}

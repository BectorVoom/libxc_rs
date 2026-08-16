//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1336/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1336(t14657: f64, t53233: f64, t13808: f64, t15278: f64, t14733: f64, t859: f64, t892: f64, t9914: f64, t13793: f64, t56320: f64, t53245: f64, t52996: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57474 = t14657 * t53233;
    let t57476 = t13808 * t15278;
    let t57480 = t14733 * t859 * t892 * t9914;
    let t57482 = t56320 * t13793;
    let t57484 = t14657 * t53245;
    let t57486 = t14657 * t52996;
    (t57474, t57476, t57480, t57482, t57484, t57486)
}

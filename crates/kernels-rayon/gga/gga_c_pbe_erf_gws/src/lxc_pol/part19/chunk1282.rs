//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1282/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1282(t13984: f64, t56320: f64, t13972: f64, t15371: f64, t12248: f64, t13780: f64, t13859: f64, t3990: f64, t9926: f64, t11401: f64, t15357: f64, t15366: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56321 = t56320 * t13984;
    let t56323 = t13972 * t15371;
    let t56333 = t13859 * t3990 * t13780 * t12248;
    let t56337 = t13859 * t3990 * t13780 * t9926;
    let t56341 = t13859 * t3990 * t13780 * t11401;
    let t56343 = t13972 * t15357;
    let t56349 = t13972 * t15366;
    (t56321, t56323, t56333, t56337, t56341, t56343, t56349)
}

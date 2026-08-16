//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1182/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1182(t3810: f64, t4039: f64, t11628: f64, t3139: f64, t4028: f64, t3862: f64, t3975: f64, t3972: f64, t13780: f64, t3742: f64, t3990: f64, t13859: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15266 = t4039 * t3810;
    let t15268 = t3139 * t11628;
    let t15269 = t4028 * t15268;
    let t15278 = t3975 * t3862;
    let t15279 = t3972 * t15278;
    let t15282 = t3990 * t13780 * t3742;
    let t15283 = t13859 * t15282;
    (t15266, t15268, t15269, t15278, t15279, t15282, t15283)
}

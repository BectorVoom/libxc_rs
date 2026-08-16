//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1187/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1187(t15247: f64, t15271: f64, t898: f64, t338: f64, t353: f64, t3862: f64, t3975: f64, t3972: f64, t13780: f64, t3742: f64, t3990: f64, t13859: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15272 = t15247 + t15271;
    let t15273 = t898 * t15272;
    let t15275 = t338 * t353 * t15273;
    let t15278 = t3975 * t3862;
    let t15279 = t3972 * t15278;
    let t15282 = t3990 * t13780 * t3742;
    let t15283 = t13859 * t15282;
    (t15272, t15273, t15275, t15278, t15279, t15282, t15283)
}

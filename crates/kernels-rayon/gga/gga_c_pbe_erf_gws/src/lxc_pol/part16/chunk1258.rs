//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1258/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1258(t13783: f64, t50998: f64, t53860: f64, t2299: f64, t371: f64, t3970: f64, t4141: f64, t9505: f64, t13917: f64, t13919: f64, t9555: f64, t14425: f64, t51563: f64) -> (f64, f64, f64, f64) {
    let t53862 = t50998 * t53860 * t13783;
    let t53865 = t3970 * t2299 * t371;
    let t53867 = t53865 * t4141 * t9505;
    let t53870 = t13917 * t13919 * t9555;
    let t53873 = t51563 * t14425;
    (t53862, t53867, t53870, t53873)
}

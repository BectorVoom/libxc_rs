//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1358/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1358(t13953: f64, t15164: f64, t11547: f64, t13917: f64, t53156: f64, t15296: f64, t3979: f64, t15300: f64, t840: f64, t11356: f64, t3965: f64, t14121: f64, t2409: f64, t39579: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57358 = t13953 * t15164;
    let t57361 = t13917 * t53156 * t11547;
    let t57371 = t3979 * t15296;
    let t57373 = t840 * t15300;
    let t57375 = t3965 * t11356;
    let t57379 = t14121 * t2409 * t39579;
    (t57358, t57361, t57371, t57373, t57375, t57379)
}

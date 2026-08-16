//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1254/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1254(t1113: f64, t28647: f64, t3972: f64, t3975: f64, t13776: f64, t38360: f64, t13781: f64, t51134: f64, t1118: f64, t3223: f64, t361: f64, t50998: f64, t874: f64) -> (f64, f64, f64, f64) {
    let t53432 = t3972 * t3975 * t1113 * t28647;
    let t53435 = t13776 * t3975 * t38360;
    let t53439 = t3972 * t13781 * t1113 * t51134;
    let t53444 = t50998 * t361 * t1118 * t874 * t3223;
    (t53432, t53435, t53439, t53444)
}

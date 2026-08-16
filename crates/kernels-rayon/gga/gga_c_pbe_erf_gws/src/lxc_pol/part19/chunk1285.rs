//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1285/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1285(t11572: f64, t50998: f64, t51066: f64, t11450: f64, t13917: f64, t51544: f64, t11889: f64, t14637: f64, t3974: f64, t3990: f64, t14001: f64, t3744: f64) -> (f64, f64, f64, f64) {
    let t56442 = t50998 * t51066 * t11572;
    let t56460 = t13917 * t51544 * t11450;
    let t56474 = t14637 * t3990 * t3974 * t11889;
    let t56476 = t14001 * t3744;
    (t56442, t56460, t56474, t56476)
}

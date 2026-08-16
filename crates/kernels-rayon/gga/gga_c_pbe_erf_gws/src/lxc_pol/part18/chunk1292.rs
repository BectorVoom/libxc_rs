//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1292/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1292(t11572: f64, t50998: f64, t51066: f64, t1144: f64, t14402: f64, t4386: f64, t1161: f64, t353: f64, t53614: f64, t859: f64, t14418: f64, t11450: f64, t13917: f64, t51544: f64) -> (f64, f64, f64, f64, f64) {
    let t56442 = t50998 * t51066 * t11572;
    let t56445 = t4386 * t1144 * t14402;
    let t56452 = t859 * t353 * t53614 * t1161;
    let t56456 = t859 * t1144 * t14418;
    let t56460 = t13917 * t51544 * t11450;
    (t56442, t56445, t56452, t56456, t56460)
}

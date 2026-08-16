//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1227/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1227(t13859: f64, t52926: f64, t9284: f64, t13972: f64, t14726: f64, t13808: f64, t14588: f64, t13772: f64, t3083: f64, t14437: f64, t2367: f64, t1113: f64, t29103: f64, t3972: f64, t3975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52959 = t13859 * t52926 * t9284;
    let t52961 = t13972 * t14726;
    let t52962 = 7.0_f64 / 2304.0_f64 * t52961;
    let t52968 = t13808 * t14588;
    let t52969 = 7.0_f64 / 1152.0_f64 * t52968;
    let t52971 = 7.0_f64 / 144.0_f64 * t3083 * t13772;
    let t52973 = 7.0_f64 / 144.0_f64 * t2367 * t14437;
    let t52976 = t3972 * t3975 * t1113 * t29103;
    (t52959, t52962, t52969, t52971, t52973, t52976)
}

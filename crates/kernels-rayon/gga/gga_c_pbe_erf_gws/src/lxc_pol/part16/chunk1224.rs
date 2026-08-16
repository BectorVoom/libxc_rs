//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1224/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1224(t13859: f64, t52926: f64, t9284: f64, t13972: f64, t14726: f64, t13808: f64, t14588: f64, t1113: f64, t29103: f64, t3972: f64, t3975: f64, t13776: f64, t3038: f64, t9504: f64) -> (f64, f64, f64, f64, f64) {
    let t52959 = t13859 * t52926 * t9284;
    let t52961 = t13972 * t14726;
    let t52968 = t13808 * t14588;
    let t52976 = t3972 * t3975 * t1113 * t29103;
    let t52982 = t13776 * t3975 * t3038 * t9504;
    (t52959, t52961, t52968, t52976, t52982)
}

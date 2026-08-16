//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1222/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1222(t14387: f64, t804: f64, t1198: f64, t2429: f64, t6926: f64, t13917: f64, t4149: f64, t9521: f64, t14765: f64, t2118: f64, t3074: f64, t6778: f64) -> (f64, f64, f64, f64) {
    let t52884 = 6.0_f64 * t804 * t14387;
    let t52887 = 12.0_f64 * t2429 * t1198 * t6926;
    let t52889 = t13917 * t4149 * t9521;
    let t52893 = t3074 * t2118 * t14765 * t6778;
    (t52884, t52887, t52889, t52893)
}

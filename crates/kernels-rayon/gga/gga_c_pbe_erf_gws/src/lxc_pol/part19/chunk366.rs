//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 366/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk366(t1140: f64, t884: f64, t1127: f64, t1138: f64, t882: f64, t339: f64) -> (f64, f64, f64) {
    let t1142 = t884 * t1140 / 48.0_f64;
    let t1143 = t1127 - t1138 - t882 - t1142;
    let t1144 = t339 * t1143;
    (t1142, t1143, t1144)
}

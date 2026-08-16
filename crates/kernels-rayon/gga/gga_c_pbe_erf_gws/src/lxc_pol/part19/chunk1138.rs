//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1138/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1138(t14046: f64, t4171: f64, t3268: f64, t4049: f64, t1158: f64, t14058: f64, t14015: f64, t3287: f64, t1140: f64, t4033: f64, t2080: f64, t3260: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14554 = t14046 * t4171;
    let t14556 = t4049 * t3268;
    let t14558 = t14058 * t1158;
    let t14560 = t14015 * t3287;
    let t14563 = t4033 * t1140;
    let t14565 = t2080 * t3260;
    (t14554, t14556, t14558, t14560, t14563, t14565)
}

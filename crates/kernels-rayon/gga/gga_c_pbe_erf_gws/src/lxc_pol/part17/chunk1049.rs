//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1049/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1049(t1153: f64, t9505: f64, t6639: f64, t9499: f64, t3252: f64, t3259: f64, t810: f64, t3258: f64, t3257: f64, t2118: f64, t814: f64, t821: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9506 = t1153 * t9505;
    let t9509 = t9499 * t6639;
    let t9512 = t3252 * t9505;
    let t9515 = t3259 * t810;
    let t9516 = t3258 * t9515;
    let t9517 = t3257 * t9516;
    let t9520 = t2118 * t814;
    let t9521 = t821 * t9520;
    (t9506, t9509, t9512, t9516, t9517, t9520, t9521)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1181/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1181(t9550: f64, t9607: f64, t2494: f64, t3222: f64, t28667: f64, t9370: f64, t9380: f64, t8546: f64, t944: f64, t3327: f64, t810: f64, t13791: f64, t2387: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36865 = t9607 * t9550;
    let t36888 = t2494 * param_a_c;
    let t36889 = t36888 * t3222;
    let t37214 = t28667 * t9370;
    let t38360 = t9607 * t9380;
    let t43260 = t8546 * t944;
    let t47184 = t3327 * t810;
    let t50884 = t2387 * t13791;
    (t36865, t36889, t37214, t38360, t43260, t47184, t50884)
}

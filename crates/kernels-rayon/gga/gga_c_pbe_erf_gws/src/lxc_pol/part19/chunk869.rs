//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 869/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk869(t3028: f64, t369: f64, t1109: f64, t931: f64, t2164: f64, t3168: f64, t2206: f64, t3191: f64, t2133: f64, t3039: f64, t1114: f64, t6187: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9053 = t3028 * t369;
    let t9056 = t1109 * t931;
    let t9086 = 7.0_f64 / 144.0_f64 * t2164 * t3168;
    let t9096 = 7.0_f64 / 24.0_f64 * t2206 * t3191;
    let t9108 = t3039 * t2133;
    let t9111 = t1114 * t6187;
    (t9053, t9056, t9086, t9096, t9108, t9111)
}

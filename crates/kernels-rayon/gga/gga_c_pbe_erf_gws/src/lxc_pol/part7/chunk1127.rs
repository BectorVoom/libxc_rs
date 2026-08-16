//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1127/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1127(t20247: f64, t2134: f64, t2100: f64, t343: f64, t874: f64, t3065: f64, t858: f64, t6678: f64, t6679: f64, t9246: f64, t6183: f64, t6647: f64) -> (f64, f64, f64, f64, f64) {
    let t20248 = t2134 * t20247;
    let t20249 = 35.0_f64 / 36.0_f64 * t20248;
    let t20251 = t2100 * t874 * t343;
    let t20253 = t3065 * t858 * t20251;
    let t20255 = t6678 * t20253 / 24.0_f64;
    let t20256 = t9246 * t6679;
    let t20257 = t6678 * t20256;
    let t20258 = 7.0_f64 / 24.0_f64 * t20257;
    let t20259 = t6183 * t6647;
    (t20249, t20251, t20255, t20258, t20259)
}

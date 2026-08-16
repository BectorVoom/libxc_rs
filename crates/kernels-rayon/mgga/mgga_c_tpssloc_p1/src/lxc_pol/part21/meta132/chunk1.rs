//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 881/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk881(t3187: f64, t3201: f64, t3166: f64, t383: f64, t1003: f64, t1058: f64, t1061: f64, t1063: f64, t3076: f64, t3180: f64, t3186: f64, t3189: f64, t3193: f64, t3197: f64, t3200: f64, t353: f64, t384: f64) -> (f64, f64, f64) {
    let t3202 = t3187 * t3201;
    let t3204 = t383 * t3166;
    let t3206 = 2.0_f64 * t1003 * t1063 + 2.0_f64 * t1058 * t3193 + t1058 * t3197 + 2.0_f64 * t1061 * t3180 + t3076 * t384 + 2.0_f64 * t3186 * t3189 - t3200 * t3202 + t3204 * t353;
    (t3202, t3204, t3206)
}

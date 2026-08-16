//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 509/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk509(t3374: f64, t440: f64, t3236: f64, t3293: f64, t1146: f64, t448: f64, t1143: f64, t300: f64, t457: f64, t697: f64, t461: f64, t221: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3375 = 1.0_f64 / t3374;
    let t3376 = t440 * t3375;
    let t3383 = 0.40256666666666666667e0_f64 * t3236;
    let t3390 = 0.137975e0_f64 * t3293;
    let t3399 = t1146 * t1146;
    let t3400 = 1.0_f64 / t3399;
    let t3401 = t440 * t3400;
    let t3402 = t448 * t448;
    let t3403 = 1.0_f64 / t3402;
    let t3411 = t300 * t1143;
    let t3426 = t697 * t457;
    let t3427 = t3426 * t461;
    let t3428 = t221 * t3427;
    (t3375, t3376, t3383, t3390, t3400, t3401, t3403, t3411, t3426, t3428)
}

//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 900/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk900(t11203: f64, t1114: f64, t2403: f64, t241: f64, t3439: f64, t407: f64, t11135: f64, t410: f64, t417: f64, t1097: f64, t3311: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11204 = 0.36514074074074074075e0_f64 * t11203;
    let t11211 = t2403 * t1114;
    let t11219 = t241 * t3439;
    let t11243 = 1.0_f64/pow_3_2(t407);
    let t11247 = 28.0_f64 / 27.0_f64 * t11135;
    let t11265 = 1.0_f64 / t410 / t417 / 4.0_f64;
    let t11274 = 1.0_f64 / t3311 / t1097;
    let t11275 = t409 * t11274;
    (t11204, t11211, t11219, t11243, t11247, t11265, t11275)
}

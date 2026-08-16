//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 924/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk924(t9218: f64, t9220: f64, t3951: f64, t604: f64, t1406: f64, t2239: f64, t584: f64, t9212: f64, t111: f64, t4025: f64, t1454: f64, t2281: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12564 = 0.3468e3_f64 * t9218;
    let t12565 = 0.56952e3_f64 * t9220;
    let t12568 = t3951 * t604;
    let t12571 = t1406 * t2239;
    let t12603 = 2.0_f64 * t584;
    let t12604 = 6.0_f64 * t9212;
    let t12725 = t4025 * t111;
    let t12747 = t2281 * t1454;
    (t12564, t12565, t12568, t12571, t12603, t12604, t12725, t12747)
}

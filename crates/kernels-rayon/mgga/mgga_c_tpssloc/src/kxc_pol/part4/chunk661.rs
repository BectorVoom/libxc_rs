//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 661/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk661(t1557: f64, t2787: f64, t912: f64, t2792: f64, t1547: f64, t2798: f64, t896: f64, t2766: f64, t2802: f64, t4335: f64, t4340: f64, t4345: f64, t4349: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4358 = 1.0_f64 * t2787 * t1557;
    let t4359 = t1557 * t912;
    let t4361 = 2.0_f64 * t2792 * t4359;
    let t4362 = t2798 * t1547;
    let t4363 = t4362 * t896;
    let t4370 = t2802 + t2766 / 9.0_f64 + t4335 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t4340 + 2.0_f64 / 3.0_f64 * t4345 - t4349 / 3.0_f64;
    (t4358, t4359, t4361, t4362, t4363, t4370)
}

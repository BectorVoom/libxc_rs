//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1099/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1099(t5727: f64, t912: f64, t2792: f64, t2844: f64, t5726: f64, t2842: f64, t4395: f64, t4399: f64, t10704: f64, t5694: f64, t10702: f64, t5743: f64, t931: f64) -> (f64, f64, f64, f64, f64) {
    let t17517 = t5727 * t912;
    let t17519 = 2.0_f64 * t2792 * t17517;
    let t17520 = t5726 * t2844;
    let t17521 = t17520 * t912;
    let t17523 = 0.16081979498692535067e2_f64 * t2842 * t17521;
    let t17524 = t4399 * t4395;
    let t17526 = 0.32163958997385070134e2_f64 * t2842 * t17524;
    let t17527 = t5694 * t10704;
    let t17528 = t17527 * t912;
    let t17530 = 0.51726012919273400301e3_f64 * t10702 * t17528;
    let t17535 = t5743 * t931;
    (t17519, t17523, t17526, t17530, t17535)
}

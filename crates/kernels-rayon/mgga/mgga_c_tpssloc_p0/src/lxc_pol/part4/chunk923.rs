//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 923/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk923(t3700: f64, t570: f64, t111: f64, t1395: f64, t5363: f64, t580: f64, t1404: f64, t1851: f64, t584: f64, t9212: f64, t9214: f64, t9216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12461 = 1.0_f64 / t3700 / t570;
    let t12524 = t1395 * t111;
    let t12541 = 2.0_f64 * t5363 * t580;
    let t12543 = 2.0_f64 * t1851 * t1404;
    let t12560 = 0.348e1_f64 * t584;
    let t12561 = 0.156e1_f64 * t9212;
    let t12562 = 0.312e1_f64 * t9214;
    let t12563 = 0.2312e3_f64 * t9216;
    (t12461, t12524, t12541, t12543, t12560, t12561, t12562, t12563)
}

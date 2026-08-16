//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1232/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1232(t101509: f64, t105462: f64, t105474: f64, t1528: f64, t17090: f64, t17092: f64, t259: f64, t29056: f64, t4268: f64, t5558: f64, t7823: f64, t7830: f64, t7842: f64, t87779: f64, t98921: f64, t98923: f64, t98927: f64) -> f64 {
    let t108412 = 12.0_f64 * t17092 * t7830 + 0.9869604401089358619e-1_f64 * t105462 + 0.49348022005446793095e-1_f64 * t87779 - 3.0_f64 * t4268 * t29056 - 6.0_f64 * t101509 * t1528 + 0.9869604401089358619e-1_f64 * t105474 + 0.23029076935875170111e0_f64 * t98921 - 0.23029076935875170111e0_f64 * t98923 + 0.49348022005446793095e-1_f64 * t98927 + 3.0_f64 * t5558 * t7823 * t259 - 3.0_f64 * t17090 * t7842;
    t108412
}

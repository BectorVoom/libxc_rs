//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1455/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1455(t111: f64, t33578: f64, t31537: f64, t7802: f64, t31540: f64, t27226: f64, t8526: f64, t24999: f64, t26114: f64, t26179: f64, t27188: f64, t31532: f64, t31726: f64, t31734: f64, t33133: f64, t4028: f64, t4073: f64, t6515: f64, t6539: f64, t672: f64, t6862: f64, t7057: f64, t7218: f64, t7458: f64, t7787: f64, t7890: f64, t8529: f64) -> (f64, f64) {
    let t122617 = t33578 * t111;
    let t122623 = 2.0_f64 * t31537 * t7802;
    let t122625 = 2.0_f64 * t31540 * t7802;
    let t122627 = 2.0_f64 * t8526 * t27226;
    let t122643 = -2.0_f64 * t122617 * t672 - 2.0_f64 * t24999 * t7057 - 2.0_f64 * t26114 * t8529 - 2.0_f64 * t26179 * t8529 - 2.0_f64 * t27188 * t6539 - 2.0_f64 * t31532 * t4073 - 2.0_f64 * t31726 * t7458 - 2.0_f64 * t31734 * t4028 + t33133 * t7218 - t6515 * t7890 - t6862 * t7787 - t122623 - t122625 - t122627;
    (t122617, t122643)
}

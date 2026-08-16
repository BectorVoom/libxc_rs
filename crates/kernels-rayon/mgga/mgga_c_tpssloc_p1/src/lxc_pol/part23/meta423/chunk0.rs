//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1250/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1250(t1057: f64, t69923: f64, t1615: f64, t883: f64, t5866: f64, t17906: f64, t4644: f64, t17607: f64, t4571: f64, t1011: f64, t1019: f64, t1040: f64, t21482: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69924 = t69923 * t1057;
    let t70100 = t1615 * t883;
    let t70122 = t5866 * t1615;
    let t70132 = t4644 * t17906;
    let t70138 = t17607 * t4571;
    let t70148 = t69923 * t1011 * t1019;
    let t70153 = t21482 * t1040;
    (t69924, t70100, t70122, t70132, t70138, t70148, t70153)
}

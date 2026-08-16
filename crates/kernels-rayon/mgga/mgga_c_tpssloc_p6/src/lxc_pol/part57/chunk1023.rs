//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1023/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1023(t5456: f64, t8518: f64, t1799: f64, t22574: f64, t26558: f64, t33221: f64, t33603: f64, t7685: f64, t1983: f64, t28834: f64, t31758: f64, t191: f64, t192: f64, t29241: f64) -> (f64, f64, f64, f64, f64) {
    let t128555 = t8518 * t5456;
    let t128562 = 12.0_f64 * t22574 * t26558 * t33221 * t1799;
    let t128564 = 6.0_f64 * t7685 * t33603;
    let t128567 = 3.0_f64 * t1983 * t31758 * t28834;
    let t128570 = t29241 * t191 * t192;
    (t128555, t128562, t128564, t128567, t128570)
}

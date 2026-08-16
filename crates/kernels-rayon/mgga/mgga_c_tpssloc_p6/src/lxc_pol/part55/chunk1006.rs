//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1006/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1006(t1215: f64, t24815: f64, t27637: f64, t1210: f64, t1734: f64, t1011: f64, t475: f64, t1218: f64, t1232: f64, t1737: f64, t1748: f64, t24685: f64, t24712: f64, t24716: f64, t24736: f64, t27604: f64, t27609: f64, t27611: f64, t27614: f64, t27617: f64, t27622: f64, t27626: f64, t27629: f64, t27636: f64, t7331: f64, t8040: f64) -> (f64, f64, f64) {
    let t27638 = t24815 * t1215;
    let t27639 = t27637 * t27638;
    let t27642 = t1210 * t1734;
    let t27643 = t1011 * t1215;
    let t27644 = t27643 * t475;
    let t27645 = t27642 * t27644;
    let t27648 = t27604 * t1232 / 432.0_f64 - 0.10093189023535097714e-3_f64 * t27609 + t27611 / 2304.0_f64 - 0.10093189023535097714e-3_f64 * t24712 + t27614 * t1218 / 1536.0_f64 - t27617 * t1232 / 2304.0_f64 + t24716 * t1737 / 1536.0_f64 - t27622 / 3456.0_f64 - t24736 * t1748 / 2304.0_f64 - t27626 / 864.0_f64 - 0.10093189023535097714e-3_f64 * t27629 * t7331 - 0.10093189023535097714e-3_f64 * t24685 * t8040 + 0.20186378047070195428e-3_f64 * t27636 * t27639 - 0.10093189023535097714e-3_f64 * t27636 * t27645;
    (t27638, t27644, t27648)
}

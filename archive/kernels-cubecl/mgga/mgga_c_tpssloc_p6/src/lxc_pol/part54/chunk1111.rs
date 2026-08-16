//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1111/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1111<F: Float>(t1215: F, t24815: F, t27637: F, t1210: F, t1734: F, t1011: F, t475: F, t1218: F, t1232: F, t1737: F, t1748: F, t24685: F, t24712: F, t24716: F, t24736: F, t27604: F, t27609: F, t27611: F, t27614: F, t27617: F, t27622: F, t27626: F, t27629: F, t27636: F, t7331: F, t8040: F) -> F {
    let t27638 = t24815 * t1215;
    let t27639 = t27637 * t27638;
    let t27642 = t1210 * t1734;
    let t27643 = t1011 * t1215;
    let t27644 = t27643 * t475;
    let t27645 = t27642 * t27644;
    let t27648 = t27604 * t1232 / F::cast_from(432.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t27609 + t27611 / F::cast_from(2304.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t24712 + t27614 * t1218 / F::cast_from(1536.0_f64) - t27617 * t1232 / F::cast_from(2304.0_f64) + t24716 * t1737 / F::cast_from(1536.0_f64) - t27622 / F::cast_from(3456.0_f64) - t24736 * t1748 / F::cast_from(2304.0_f64) - t27626 / F::cast_from(864.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t27629 * t7331 - F::cast_from(0.10093189023535097714e-3_f64) * t24685 * t8040 + F::cast_from(0.20186378047070195428e-3_f64) * t27636 * t27639 - F::cast_from(0.10093189023535097714e-3_f64) * t27636 * t27645;
    t27648
}

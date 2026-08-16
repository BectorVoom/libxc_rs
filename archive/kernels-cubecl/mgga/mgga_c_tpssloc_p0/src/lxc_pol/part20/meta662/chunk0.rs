//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2482/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2482<F: Float>(t1102: F, t3279: F, t14801: F, t14804: F, t43727: F, t43729: F, t43748: F, t43750: F, t50824: F, t50827: F, t50828: F, t50832: F, t50834: F) -> (F, F, F) {
    let t50836 = t1102 * t3279;
    let t50837 = t14801 * t50836;
    let t50839 = t14804 * t50836;
    let t50845 = F::cast_from(0.149013e1_f64) * t50824 + t50827 - F::cast_from(0.30192500000000000001e0_f64) * t50828 + F::cast_from(0.301925e0_f64) * t50832 - F::cast_from(0.31310740740740740741e0_f64) * t50834 + F::cast_from(0.58258125e1_f64) * t50837 - F::cast_from(0.1237865625e0_f64) * t50839 + F::cast_from(0.20128333333333333334e0_f64) * t43727 - F::cast_from(0.60385000000000000002e0_f64) * t43729 - F::cast_from(0.26837777777777777778e0_f64) * t43748 - F::cast_from(0.11182407407407407408e0_f64) * t43750;
    (t50837, t50839, t50845)
}

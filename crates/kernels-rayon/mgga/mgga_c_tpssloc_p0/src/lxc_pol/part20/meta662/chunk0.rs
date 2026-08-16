//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2482/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2482(t1102: f64, t3279: f64, t14801: f64, t14804: f64, t43727: f64, t43729: f64, t43748: f64, t43750: f64, t50824: f64, t50827: f64, t50828: f64, t50832: f64, t50834: f64) -> (f64, f64, f64) {
    let t50836 = t1102 * t3279;
    let t50837 = t14801 * t50836;
    let t50839 = t14804 * t50836;
    let t50845 = 0.149013e1_f64 * t50824 + t50827 - 0.30192500000000000001e0_f64 * t50828 + 0.301925e0_f64 * t50832 - 0.31310740740740740741e0_f64 * t50834 + 0.58258125e1_f64 * t50837 - 0.1237865625e0_f64 * t50839 + 0.20128333333333333334e0_f64 * t43727 - 0.60385000000000000002e0_f64 * t43729 - 0.26837777777777777778e0_f64 * t43748 - 0.11182407407407407408e0_f64 * t43750;
    (t50837, t50839, t50845)
}

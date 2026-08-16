//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2530/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2530(t3266: f64, t51246: f64, t11189: f64, t1657: f64, t11192: f64, t50826: f64, t43727: f64, t43729: f64, t43748: f64, t43750: f64, t50824: f64, t50828: f64, t50832: f64, t50834: f64, t50837: f64, t50839: f64) -> (f64, f64, f64) {
    let t51248 = 6.0_f64 * t51246 * t3266;
    let t51249 = t1657 * t11189;
    let t51251 = 0.96491876992155210402e2_f64 * t51249 * t11192;
    let t51257 = 0.68863333333333333332e0_f64 * t50826;
    let t51267 = 0.187551e1_f64 * t50824 + t51257 - 0.51647499999999999999e0_f64 * t50828 + 0.516475e0_f64 * t50832 - 0.53560370370370370369e0_f64 * t50834 + 0.794188125e1_f64 * t50837 - 0.473371875e0_f64 * t50839 + 0.34431666666666666666e0_f64 * t43727 - 0.103295e1_f64 * t43729 - 0.45908888888888888888e0_f64 * t43748 - 0.19128703703703703703e0_f64 * t43750;
    (t51248, t51251, t51267)
}

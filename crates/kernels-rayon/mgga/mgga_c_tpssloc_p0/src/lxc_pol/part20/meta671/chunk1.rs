//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2522/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2522(t11131: f64, t4869: f64, t11427: f64, t14850: f64, t50826: f64, t43727: f64, t43729: f64, t43748: f64, t43750: f64, t50824: f64, t50828: f64, t50832: f64, t50834: f64, t50837: f64, t50839: f64) -> (f64, f64, f64) {
    let t51131 = 0.35089341735807877242e1_f64 * t4869 * t11131;
    let t51133 = 6.0_f64 * t14850 * t11427;
    let t51137 = 0.39862222222222222223e0_f64 * t50826;
    let t51147 = 0.147882e1_f64 * t50824 + t51137 - 0.29896666666666666667e0_f64 * t50828 + 0.29896666666666666667e0_f64 * t50832 - 0.31003950617283950619e0_f64 * t50834 + 0.427258125e1_f64 * t50837 - 0.230371875e0_f64 * t50839 + 0.19931111111111111112e0_f64 * t43727 - 0.59793333333333333333e0_f64 * t43729 - 0.26574814814814814816e0_f64 * t43748 - 0.11072839506172839506e0_f64 * t43750;
    (t51131, t51133, t51147)
}

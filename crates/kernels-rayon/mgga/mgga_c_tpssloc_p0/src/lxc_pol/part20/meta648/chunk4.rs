//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2383/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2383(t47730: f64, t41656: f64, t41658: f64, t41660: f64, t47732: f64, t47736: f64, t47738: f64, t47744: f64, t47748: f64, t48098: f64, t48101: f64, t48103: f64) -> f64 {
    let t48924 = 0.39862222222222222223e0_f64 * t47730;
    let t48934 = 0.16431333333333333333e0_f64 * t48098 - 0.82156666666666666667e-1_f64 * t48101 - t48924 + 0.29896666666666666667e0_f64 * t47732 - 0.29896666666666666667e0_f64 * t47736 + 0.17938e1_f64 * t47738 + 0.39862222222222222223e1_f64 * t47744 + 0.71752e1_f64 * t47748 + 0.24342716049382716049e0_f64 * t48103 - 0.39862222222222222224e0_f64 * t41656 - 0.26574814814814814816e0_f64 * t41658 + 0.11072839506172839506e0_f64 * t41660;
    t48934
}

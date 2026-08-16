//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2410/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2410(t47730: f64, t41656: f64, t41658: f64, t41660: f64, t47732: f64, t47736: f64, t47738: f64, t47744: f64, t47748: f64, t48098: f64, t48101: f64, t48103: f64) -> f64 {
    let t49322 = 0.68863333333333333332e0_f64 * t47730;
    let t49332 = 0.20839e0_f64 * t48098 - 0.104195e0_f64 * t48101 - t49322 + 0.51647499999999999999e0_f64 * t47732 - 0.516475e0_f64 * t47736 + 0.309885e1_f64 * t47738 + 0.68863333333333333334e1_f64 * t47744 + 0.123954e2_f64 * t47748 + 0.30872592592592592592e0_f64 * t48103 - 0.68863333333333333332e0_f64 * t41656 - 0.45908888888888888888e0_f64 * t41658 + 0.19128703703703703703e0_f64 * t41660;
    t49332
}

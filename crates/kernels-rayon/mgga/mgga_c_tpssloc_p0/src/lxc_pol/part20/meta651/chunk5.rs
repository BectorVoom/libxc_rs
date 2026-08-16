//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2399/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2399(t47730: f64, t41656: f64, t41658: f64, t41660: f64, t47732: f64, t47736: f64, t47738: f64, t47744: f64, t47748: f64, t48098: f64, t48101: f64, t48103: f64) -> f64 {
    let t49144 = 0.40256666666666666668e0_f64 * t47730;
    let t49154 = 0.16557e0_f64 * t48098 - 0.82785e-1_f64 * t48101 - t49144 + 0.30192500000000000001e0_f64 * t47732 - 0.301925e0_f64 * t47736 + 0.181155e1_f64 * t47738 + 0.40256666666666666666e1_f64 * t47744 + 0.72462e1_f64 * t47748 + 0.24528888888888888889e0_f64 * t48103 - 0.40256666666666666667e0_f64 * t41656 - 0.26837777777777777778e0_f64 * t41658 + 0.11182407407407407408e0_f64 * t41660;
    t49154
}

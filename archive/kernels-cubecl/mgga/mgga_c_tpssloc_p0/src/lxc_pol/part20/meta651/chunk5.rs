//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2399/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2399<F: Float>(t47730: F, t41656: F, t41658: F, t41660: F, t47732: F, t47736: F, t47738: F, t47744: F, t47748: F, t48098: F, t48101: F, t48103: F) -> F {
    let t49144 = F::cast_from(0.40256666666666666668e0_f64) * t47730;
    let t49154 = F::cast_from(0.16557e0_f64) * t48098 - F::cast_from(0.82785e-1_f64) * t48101 - t49144 + F::cast_from(0.30192500000000000001e0_f64) * t47732 - F::cast_from(0.301925e0_f64) * t47736 + F::cast_from(0.181155e1_f64) * t47738 + F::cast_from(0.40256666666666666666e1_f64) * t47744 + F::cast_from(0.72462e1_f64) * t47748 + F::cast_from(0.24528888888888888889e0_f64) * t48103 - F::cast_from(0.40256666666666666667e0_f64) * t41656 - F::cast_from(0.26837777777777777778e0_f64) * t41658 + F::cast_from(0.11182407407407407408e0_f64) * t41660;
    t49154
}

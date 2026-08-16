//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2383/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2383<F: Float>(t47730: F, t41656: F, t41658: F, t41660: F, t47732: F, t47736: F, t47738: F, t47744: F, t47748: F, t48098: F, t48101: F, t48103: F) -> F {
    let t48924 = F::cast_from(0.39862222222222222223e0_f64) * t47730;
    let t48934 = F::cast_from(0.16431333333333333333e0_f64) * t48098 - F::cast_from(0.82156666666666666667e-1_f64) * t48101 - t48924 + F::cast_from(0.29896666666666666667e0_f64) * t47732 - F::cast_from(0.29896666666666666667e0_f64) * t47736 + F::cast_from(0.17938e1_f64) * t47738 + F::cast_from(0.39862222222222222223e1_f64) * t47744 + F::cast_from(0.71752e1_f64) * t47748 + F::cast_from(0.24342716049382716049e0_f64) * t48103 - F::cast_from(0.39862222222222222224e0_f64) * t41656 - F::cast_from(0.26574814814814814816e0_f64) * t41658 + F::cast_from(0.11072839506172839506e0_f64) * t41660;
    t48934
}

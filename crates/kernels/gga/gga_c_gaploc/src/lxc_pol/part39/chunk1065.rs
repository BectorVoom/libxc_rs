//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1065/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1065<F: Float>(t41542: F, t10105: F, t2969: F, t11127: F, t7324: F, t3511: F, t7822: F, t13235: F, t16710: F, t841: F, t11125: F, t2592: F) -> (F, F, F, F, F, F) {
    let t44186 = F::cast_from(0.25561950635947166451e0_f64) * t41542;
    let t44194 = t2969 * t10105;
    let t44196 = t7324 * t11127;
    let t44198 = t7822 * t3511;
    let t44202 = F::cast_from(24.0_f64) * t16710 * t13235 * t841;
    let t44203 = t2592 * t11125;
    (t44186, t44194, t44196, t44198, t44202, t44203)
}

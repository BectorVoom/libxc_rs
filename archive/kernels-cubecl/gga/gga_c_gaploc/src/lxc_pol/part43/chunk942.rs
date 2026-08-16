//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 942/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk942<F: Float>(t41518: F, t41538: F, t41542: F, t10105: F, t2969: F, t13235: F, t16710: F, t841: F, t2728: F, t3459: F, t5559: F, t13241: F) -> (F, F, F, F, F, F, F) {
    let t44181 = F::cast_from(0.59584149919750711116e-1_f64) * t41518;
    let t44185 = F::cast_from(0.17041300423964777634e0_f64) * t41538;
    let t44186 = F::cast_from(0.25561950635947166451e0_f64) * t41542;
    let t44194 = t2969 * t10105;
    let t44202 = F::cast_from(24.0_f64) * t16710 * t13235 * t841;
    let t44207 = F::cast_from(12.0_f64) * t5559 * t3459 * t2728;
    let t44221 = F::cast_from(6.0_f64) * t5559 * t13241 * t841;
    (t44181, t44185, t44186, t44194, t44202, t44207, t44221)
}

//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 932/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk932<F: Float>(t1098: F, t4627: F, t41: F, t85: F, t8565: F, t4589: F, t4672: F, t1758: F, t3251: F, t313: F, t4625: F, t1762: F, t1071: F, t1109: F, t10415: F, t1670: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14235 = 0.19711289e-2 * t1098 * t4627;
    let t14249 = t85 * t8565 * t41;
    let t14250 = t14249 * t4589;
    let t14260 = 0.13140859333333333333e-2 * t1098 * t4672;
    let t14272 = t3251 * t1758;
    let t14282 = t313 * t4625;
    let t14299 = t3251 * t1762;
    let t14301 = t1109 * t1071;
    let t14316 = t10415 * t1670;
    (t14235, t14249, t14250, t14260, t14272, t14282, t14299, t14301, t14316)
}

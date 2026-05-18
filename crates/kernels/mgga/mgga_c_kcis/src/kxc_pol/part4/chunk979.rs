//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 979/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk979<F: Float>(t3066: F, t331: F, t10112: F, t313: F, t1031: F, t1068: F, t1046: F, t3054: F, t3069: F, t1027: F, t3097: F, t308: F, t9758: F) -> (F, F, F, F, F, F, F) {
    let t10184 = t331 * t3066;
    let t10187 = F::new(0.14055920378328537299e-1) * t10112 * t313;
    let t10188 = t1068 * t1031;
    let t10190 = t3054 * t1046;
    let t10192 = t331 * t3069;
    let t10194 = t1027 * t3097;
    let t10199 = t9758 * t308;
    (t10184, t10187, t10188, t10190, t10192, t10194, t10199)
}

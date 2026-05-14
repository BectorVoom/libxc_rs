//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 587/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk587<F: Float>(t3245: F, t361: F, t1014: F, t1127: F, t126: F, t88: F, t85: F) -> (F, F, F, F, F) {
    let t3246 = t3245 * t361;
    let t3247 = 0.55273148148148148147e-3 * t3246;
    let t3248 = t1014 * t1127;
    let t3250 = t126 * t88;
    let t3251 = t85 * t3250;
    (t3246, t3247, t3248, t3250, t3251)
}

//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 929/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk929<F: Float>(t12265: F, t3728: F, t4126: F, t509: F, t86: F, t9526: F, t1499: F, t3724: F, t1491: F, t1495: F, t4161: F, t1360: F, t3960: F, t1460: F, t3245: F, t10470: F, t558: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12266 = t12265 * sigma2;
    let t12271 = t3728 * t4126;
    let t12274 = t86 * t9526 * t509;
    let t12275 = t12274 * t1499;
    let t12277 = t3728 * t3724;
    let t12279 = t12274 * t1491;
    let t12281 = t4161 * t1495;
    let t12286 = t1360 * t3960;
    let t12303 = t3245 * t1460;
    let t12305 = t10470 * t558;
    (t12266, t12271, t12274, t12275, t12277, t12279, t12281, t12286, t12303, t12305)
}

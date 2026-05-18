//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 812/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk812<F: Float>(t4121: F, t491: F, t1457: F, t509: F, t86: F, t9526: F, t1499: F, t1491: F, t1495: F, t4161: F, t1360: F, t3960: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12240 = t4121 * sigma2;
    let t12241 = t12240 * t491;
    let t12265 = t1457 * t4121;
    let t12266 = t12265 * sigma2;
    let t12274 = t86 * t9526 * t509;
    let t12275 = t12274 * t1499;
    let t12279 = t12274 * t1491;
    let t12281 = t4161 * t1495;
    let t12286 = t1360 * t3960;
    (t12240, t12241, t12265, t12266, t12274, t12275, t12279, t12281, t12286)
}

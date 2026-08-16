//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 793/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk793<F: Float>(t4121: F, t491: F, t1363: F, t3951: F, t11913: F, t4174: F, t3728: F, t4138: F, t1457: F, t4126: F, t509: F, t86: F, t9526: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12240 = t4121 * sigma2;
    let t12241 = t12240 * t491;
    let t12246 = t3951 * t1363;
    let t12251 = t11913 * t4174;
    let t12263 = t3728 * t4138;
    let t12265 = t1457 * t4121;
    let t12266 = t12265 * sigma2;
    let t12271 = t3728 * t4126;
    let t12274 = t86 * t9526 * t509;
    (t12240, t12241, t12246, t12251, t12263, t12265, t12266, t12271, t12274)
}

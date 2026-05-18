//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 426/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk426<F: Float>(t214: F, t363: F, t1054: F, t378: F, t1084: F, t1102: F, t1106: F, t245: F, t1110: F, t395: F, t1077: F, t1076: F, t394: F) -> (F, F, F, F, F, F) {
    let t4249 = t214 * t363;
    let t4252 = F::new(0.71233333333333333332e-1) * t1054 * t4249 * t378;
    let t4255 = F::new(0.53424999999999999999e-1) * t1054 * t1084 * t1102;
    let t4256 = t245 * t1106;
    let t4259 = F::new(0.85917975471764868594e0) * t1054 * t4256 * t1110;
    let t4260 = t214 * t395;
    let t4267 = t245 * t1077;
    let t4272 = F::new(1.0) / t1076 / t394;
    (t4252, t4255, t4259, t4260, t4267, t4272)
}

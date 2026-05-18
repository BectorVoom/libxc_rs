//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 244/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk244<F: Float>(t1072: F, t402: F, t394: F, t158: F, t166: F, t1062: F, t245: F, t363: F, t1054: F, t378: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1073 = t1072 * t402;
    let t1076 = t394 * t394;
    let t1077 = F::new(1.0) / t1076;
    let t1078 = t158 * t1077;
    let t1079 = t166 * t166;
    let t1080 = F::new(1.0) / t1079;
    let t1081 = t1062 * t1080;
    let t1084 = t245 * t363;
    let t1087 = F::new(0.35616666666666666666e-1) * t1054 * t1084 * t378;
    (t1073, t1076, t1077, t1078, t1079, t1080, t1081, t1084, t1087)
}

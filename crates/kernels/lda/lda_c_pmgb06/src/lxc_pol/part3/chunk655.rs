//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 655/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk655<F: Float>(t12: F, t3922: F, t764: F, t1: F, t1079: F, t1080: F, t1083: F, t14: F, t2133: F, t2136: F, t247: F, t395: F, t4382: F, t257: F, t4499: F, zeta_threshold: F) -> (F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t4500 = t3922 * t764;
    let t4503 = t1079 * t1;
    let t4513 = piecewise3(t13, 0.0, -8.0 / 27.0 * t4500 * t1080 - 16.0 / 9.0 * t4503 * t4382 + 4.0 / 9.0 * t2133 * t1083 - 8.0 / 3.0 * t14 * t395 + 8.0 * t2136 * t247);
    let t4515 = (t4499 + t4513) * t257;
    (t4500, t4503, t4515)
}

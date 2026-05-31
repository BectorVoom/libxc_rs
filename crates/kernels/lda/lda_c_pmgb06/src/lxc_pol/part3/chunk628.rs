//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 628/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk628<F: Float>(t3564: F, t3619: F, t1289: F, t377: F, t1295: F, t374: F, t376: F, t67: F, t1297: F, t384: F, t1309: F, t1180: F, t56: F, t69: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3620 = t3564 + t3619;
    let t3622 = t1289 * t377;
    let t3625 = t374 * t1295;
    let t3630 = t376 * t376;
    let t3631 = F::cast_from(1.0_f64) / t3630;
    let t3632 = t67 * t3631;
    let t3633 = t1297 * t384;
    let t3636 = t384 * t1309;
    let t3643 = F::cast_from(0.8940581481481481_f64) * t69 * t1180 * t56;
    (t3620, t3622, t3625, t3630, t3631, t3632, t3633, t3636, t3643)
}

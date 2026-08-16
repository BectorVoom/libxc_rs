//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 881/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk881<F: Float>(t248: F, t258: F, t8887: F, t8925: F, t8990: F, t9033: F, t1200: F, t718: F, t2803: F, t81: F, t199: F, t2813: F, t566: F) -> (F, F, F, F, F) {
    let t9037 = t248 * t258 * (t8887 + t8925 + t8990 + t9033);
    let t9045 = t718 * t1200;
    let t9047 = t81 * t2803;
    let t9048 = t9047 * t199;
    let t9050 = t2813 * t566;
    (t9037, t9045, t9047, t9048, t9050)
}

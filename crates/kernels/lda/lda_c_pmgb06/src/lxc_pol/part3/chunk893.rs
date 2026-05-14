//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 893/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk893<F: Float>(t9404: F, t1560: F, t5220: F, t1420: F, t5198: F, t136: F, t1540: F, t1968: F, t439: F, t9408: F, t9410: F, t9412: F, t9413: F, t9417: F, t9418: F, t9422: F) -> (F, F, F, F, F) {
    let t11951 = 2.0 / 45.0 * t9404;
    let t11952 = t5220 * t1560;
    let t11953 = 4.0 / 45.0 * t11952;
    let t11955 = 2.0 / 5.0 * t1420 * t5198;
    let t11959 = t439 * t136 * t1540 * t1968 / 5.0;
    let t11963 = -t11951 - t11953 + t11955 + t11959 - t9408 + t9410 + t9412 + 2.0 / 9.0 * t9413 - t9417 + 4.0 / 3.0 * t9418 + 2.0 * t9422;
    (t11951, t11953, t11955, t11959, t11963)
}

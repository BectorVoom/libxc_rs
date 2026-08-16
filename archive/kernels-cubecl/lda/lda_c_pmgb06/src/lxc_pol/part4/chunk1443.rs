//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1443/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1443<F: Float>(t2174: F, t868: F, t2422: F, t718: F, t199: F, t5988: F, t1795: F, t1808: F, t1329: F, t391: F, t6939: F, t18061: F) -> (F, F, F, F, F, F, F) {
    let t18418 = t2174 * t868;
    let t18420 = t718 * t2422;
    let t18422 = t5988 * t199;
    let t18424 = t1795 * t1808;
    let t18426 = t1329 * t2422;
    let t18428 = t391 * t6939;
    let t18430 = t18061 * t199;
    (t18418, t18420, t18422, t18424, t18426, t18428, t18430)
}

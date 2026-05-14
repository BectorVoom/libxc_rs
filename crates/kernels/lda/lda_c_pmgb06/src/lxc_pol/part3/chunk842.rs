//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 842/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk842<F: Float>(t1777: F, t1786: F, t1789: F, t409: F, t247: F, t4344: F, t749: F, t327: F, t317: F, t321: F, t4001: F, t934: F, t97: F, t27: F, t2767: F, t927: F) -> (F, F, F, F) {
    let t10964 = t409 * t1777 * t1786 * t1789;
    let t10967 = t247 * t749 * t4344;
    let t10970 = t327 * t327;
    let t10976 = 0.3407285805772476 * t4001 * t321 / t10970 * t317 * t97 * t934;
    let t10980 = t927 * t1786 * t27 * t2767;
    (t10964, t10967, t10976, t10980)
}

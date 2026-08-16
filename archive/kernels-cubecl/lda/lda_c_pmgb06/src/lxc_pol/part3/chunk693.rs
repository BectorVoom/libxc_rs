//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 693/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk693<F: Float>(t1322: F, t2257: F, t1234: F, t23: F, t384: F, t769: F, t1152: F, t123: F, t868: F, t740: F, t794: F, t199: F) -> (F, F, F, F, F, F) {
    let t4398 = t2257 * t1322;
    let t4405 = t1234 * t23;
    let t4414 = t384 * t769;
    let t4427 = t123 * t1152 * t868;
    let t4429 = t740 * t794;
    let t4431 = t123 * t4429 * t199;
    (t4398, t4405, t4414, t4427, t4429, t4431)
}

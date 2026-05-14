//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 591/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk591<F: Float>(t1554: F, t843: F, t161: F, t1555: F, t831: F, t1548: F, t802: F, t1547: F, t814: F, t132: F, t130: F, t160: F) -> (F, F, F, F, F, F, F) {
    let t5044 = t1554 * t843;
    let t5045 = t161 * t5044;
    let t5047 = t831 * t1555;
    let t5049 = t802 * t1548;
    let t5051 = t1547 * t814;
    let t5052 = t132 * t5051;
    let t5065 = t160 * t130;
    (t5044, t5045, t5047, t5049, t5051, t5052, t5065)
}

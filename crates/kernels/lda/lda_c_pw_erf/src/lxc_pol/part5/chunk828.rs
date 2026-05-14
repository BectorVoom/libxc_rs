//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 828/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk828<F: Float>(t10042: F, t2061: F, t590: F, t1333: F, t191: F, t205: F, t190: F, t212: F, t9821: F, t4233: F, t598: F, t226: F, t4606: F, t5021: F, t7: F, t1458: F, t155: F) -> (F, F, F, F, F, F, F) {
    let t10195 = 0.3732469135802469 * t10042;
    let t10202 = t2061 * t590;
    let t10216 = t191 / t205 / t1333;
    let t10225 = 0.10864197530864197 * t190 * t9821 * t212;
    let t10278 = t598 * t4233;
    let t10286 = 4.0 / 3.0 * t226 * (-4.277777777777778 * t4606 + 220.0 / 81.0 * t5021) * M_PI * t7;
    let t10313 = t155 * t1458;
    (t10195, t10202, t10216, t10225, t10278, t10286, t10313)
}

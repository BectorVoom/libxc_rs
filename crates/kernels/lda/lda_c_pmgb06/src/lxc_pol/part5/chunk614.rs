//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 614/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk614<F: Float>(t1798: F, t395: F, t113: F, t301: F, t2174: F, t413: F, t26: F, t789: F, t329: F) -> (F, F, F, F, F) {
    let t5575 = t395 * t1798;
    let t5578 = 0.0005811348303577384 * t5575 * t113 * t301;
    let t5580 = t2174 * t413 * t301;
    let t5582 = t26 * t789;
    let t5583 = t329 * t5582;
    (t5575, t5578, t5580, t5582, t5583)
}

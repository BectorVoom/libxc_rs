//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 924/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk924<F: Float>(t2789: F, t301: F, t718: F, t1135: F, t1183: F, t113: F, t2803: F, t395: F, t3982: F, t413: F, t26: F, t4038: F) -> (F, F, F, F, F) {
    let t10614 = F::new(0.0011622696607154768) * t718 * t2789 * t301;
    let t10617 = F::new(0.008135887625008338) * t1135 * t1183 * t301;
    let t10620 = t395 * t2803 * t113 * t301;
    let t10623 = t3982 * t413 * t301;
    let t10625 = t4038 * t26;
    (t10614, t10617, t10620, t10623, t10625)
}

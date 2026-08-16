//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1003/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1003<F: Float>(t2954: F, t739: F, t9763: F, t11: F, t9836: F, t34: F, t3518: F, t940: F, t1953: F, t3536: F, t3476: F, t1243: F) -> (F, F, F, F, F, F) {
    let t11724 = t9763 * t739 * t2954;
    let t11726 = t11 * t9836 * t11724;
    let t11729 = t3518 * t34 * t940;
    let t11731 = t1953 * t3536 * t11729;
    let t11746 = t3476 * t34 * t940;
    let t11748 = t1953 * t1243 * t11746;
    (t11724, t11726, t11729, t11731, t11746, t11748)
}

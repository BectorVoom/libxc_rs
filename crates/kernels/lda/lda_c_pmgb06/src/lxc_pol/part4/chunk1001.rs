//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1001/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1001<F: Float>(t242: F, t2786: F, t30: F, t1041: F, t1043: F, t3697: F, t632: F, t2801: F, t687: F, t2799: F, t654: F, t3891: F, t643: F) -> (F, F, F, F, F, F) {
    let t8837 = F::new(0.011483599538271605) * t30 * t2786 * t242;
    let t8841 = F::new(64.32791799477015) * t1041 * t3697 * t1043 * t632;
    let t8842 = t2801 * t687;
    let t8844 = t2799 * t654;
    let t8846 = t2801 * t654;
    let t8853 = F::new(16.0) * t643 * t3891;
    (t8837, t8841, t8842, t8844, t8846, t8853)
}

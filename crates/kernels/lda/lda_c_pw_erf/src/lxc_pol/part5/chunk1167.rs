//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1167/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1167<F: Float>(t656: F, t8033: F, t17040: F, t12956: F, t1995: F, t3965: F, t4495: F, t2146: F, t6970: F, t6974: F, t1475: F, t571: F, t7478: F) -> (F, F, F, F, F, F) {
    let t21336 = t8033 * t656;
    let t21338 = F::new(16.0) / F::new(135.0) * t17040;
    let t21342 = F::new(32.0) / F::new(15.0) * t3965 * t12956 * t1995 * t4495;
    let t21344 = F::new(12.0) / F::new(5.0) * t2146 * t6970;
    let t21346 = F::new(8.0) / F::new(5.0) * t2146 * t6974;
    let t21348 = t571 * t1475 * t7478;
    (t21336, t21338, t21342, t21344, t21346, t21348)
}

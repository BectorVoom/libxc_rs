//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1079/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1079<F: Float>(t5386: F, t591: F, t4111: F, t5391: F, t138: F, t4676: F, t9175: F, t1869: F, t8337: F, t1830: F, t810: F, t4641: F, t4656: F) -> (F, F, F, F, F, F) {
    let t12310 = t5386 * t591;
    let t12312 = t5391 * t4111;
    let t12325 = t138 * t9175 * t4676;
    let t12329 = t8337 * t1869;
    let t12337 = t1830 * t810;
    let t12354 = t4641 * t4656;
    (t12310, t12312, t12325, t12329, t12337, t12354)
}

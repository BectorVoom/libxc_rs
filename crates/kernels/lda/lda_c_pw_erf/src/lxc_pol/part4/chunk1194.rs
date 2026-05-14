//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1194/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1194<F: Float>(t3965: F, t4628: F, t6762: F, t12475: F, t1972: F, t34: F, t4722: F, t1967: F, t5146: F, t2328: F, t3966: F, t1314: F, t12136: F, t4484: F, t16858: F, t4491: F) -> (F, F, F, F, F, F) {
    let t17628 = 32.0 / 15.0 * t3965 * t6762 * t4628;
    let t17632 = 64.0 / 45.0 * t12475 * t4722 * t34 * t1972;
    let t17636 = 32.0 / 27.0 * t12475 * t5146 * t34 * t1967;
    let t17637 = t3966 * t2328;
    let t17640 = 16.0 / 45.0 * t3965 * t17637 * t1314;
    let t17642 = 32.0 / 45.0 * t12136 * t4484;
    let t17644 = 32.0 / 45.0 * t16858 * t4491;
    (t17628, t17632, t17636, t17640, t17642, t17644)
}

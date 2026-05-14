//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 980/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk980<F: Float>(t12031: F, t20737: F, t4488: F, t17637: F, t2030: F, t3965: F, t1972: F, t2328: F, t4722: F, t1967: F, t5146: F, t12136: F, t6771: F, t16858: F, t6713: F, t6717: F) -> (F, F, F, F, F, F, F) {
    let t20740 = 32.0 / 27.0 * t4488 * t12031 * t20737;
    let t20743 = 8.0 / 15.0 * t3965 * t17637 * t2030;
    let t20747 = 16.0 / 15.0 * t3965 * t4722 * t2328 * t1972;
    let t20751 = 8.0 / 9.0 * t3965 * t5146 * t2328 * t1967;
    let t20753 = 16.0 / 15.0 * t12136 * t6771;
    let t20755 = 16.0 / 15.0 * t16858 * t6713;
    let t20757 = 16.0 / 15.0 * t16858 * t6717;
    (t20740, t20743, t20747, t20751, t20753, t20755, t20757)
}

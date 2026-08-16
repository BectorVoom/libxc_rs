//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1114/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1114<F: Float>(t12031: F, t20737: F, t4488: F, t17637: F, t2030: F, t3965: F, t1972: F, t2328: F, t4722: F, t1967: F, t5146: F, t12136: F, t6771: F) -> (F, F, F, F, F) {
    let t20740 = F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t4488 * t12031 * t20737;
    let t20743 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3965 * t17637 * t2030;
    let t20747 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3965 * t4722 * t2328 * t1972;
    let t20751 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3965 * t5146 * t2328 * t1967;
    let t20753 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t12136 * t6771;
    (t20740, t20743, t20747, t20751, t20753)
}

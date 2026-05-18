//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 921/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk921<F: Float>(t1634: F, t1638: F, t635: F, t1125: F, t602: F, t603: F, t4192: F, t4207: F, t163: F, t169: F, t234: F, t2817: F) -> (F, F, F, F) {
    let t10715 = F::new(0.04472697096444135) * t1638 * t635 * t1634;
    let t10718 = F::new(0.2244364134416412) * t602 * t1125 * t603;
    let t10719 = t4192 * t4207;
    let t10749 = F::new(0.4097848972398244) * t169 * t2817 * t234 * t163;
    (t10715, t10718, t10719, t10749)
}

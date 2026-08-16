//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1198/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1198<F: Float>(t17664: F, t595: F, t7676: F, t544: F, t7661: F, t184: F, t202: F, t7674: F, t551: F, t17684: F, t17687: F, t17690: F) -> (F, F, F, F, F, F, F) {
    let t21717 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t17664;
    let t21719 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t7676 * t595;
    let t21721 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t7661 * t544;
    let t21723 = t202 * t7674 * t184;
    let t21725 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t21723 * t551;
    let t21726 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17684;
    let t21727 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t17687;
    let t21728 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t17690;
    (t21717, t21719, t21721, t21725, t21726, t21727, t21728)
}

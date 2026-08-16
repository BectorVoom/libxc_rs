//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1095/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1095<F: Float>(t12535: F, t441: F, t5075: F, t13021: F, t5094: F, t12683: F, t5082: F, t5087: F, t13005: F, t13009: F, t13012: F, t13015: F, t13018: F, t13024: F, t13030: F, t13034: F, t13038: F, t13041: F) -> (F, F, F) {
    let t13043 = t5075 * t12535 * t441;
    let t13046 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t13043 * t5094 * t13021;
    let t13047 = t12683 * t5082;
    let t13049 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13047 * t5087;
    let t13050 = t13005 + t13009 + t13012 - t13015 + t13018 - t13024 + t13030 - t13034 - t13038 - t13041 + t13046 - t13049;
    (t13046, t13049, t13050)
}

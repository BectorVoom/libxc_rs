//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1474/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1474<F: Float>(t123: F, t6939: F, t722: F, t10895: F, t14707: F, t14709: F, t14712: F, t14723: F, t14726: F, t14741: F, t14744: F, t15059: F, t4435: F, t4464: F, t81: F, t868: F, t912: F) -> F {
    let t19031 = t123 * t722 * t6939;
    let t19041 = -F::cast_from(0.06367133154935875_f64) * t123 * t4464 * t868 - F::cast_from(2.55960325162461_f64) * t14707 + F::cast_from(1.279801625812305_f64) * t14709 + F::cast_from(0.05332506774217938_f64) * t81 * t15059 + F::cast_from(0.10611888591559791_f64) * t19031 + F::cast_from(0.10611888591559791_f64) * t14712 - F::cast_from(0.06367133154935875_f64) * t123 * t912 * t4435 + F::cast_from(0.10611888591559791_f64) * t14723 + F::cast_from(0.21223777183119583_f64) * t14726 + F::cast_from(0.10611888591559791_f64) * t14741 + F::cast_from(0.21223777183119583_f64) * t14744 + t10895;
    t19041
}

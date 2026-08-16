//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1474/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1474(t123: f64, t6939: f64, t722: f64, t10895: f64, t14707: f64, t14709: f64, t14712: f64, t14723: f64, t14726: f64, t14741: f64, t14744: f64, t15059: f64, t4435: f64, t4464: f64, t81: f64, t868: f64, t912: f64) -> f64 {
    let t19031 = t123 * t722 * t6939;
    let t19041 = -0.06367133154935875_f64 * t123 * t4464 * t868 - 2.55960325162461_f64 * t14707 + 1.279801625812305_f64 * t14709 + 0.05332506774217938_f64 * t81 * t15059 + 0.10611888591559791_f64 * t19031 + 0.10611888591559791_f64 * t14712 - 0.06367133154935875_f64 * t123 * t912 * t4435 + 0.10611888591559791_f64 * t14723 + 0.21223777183119583_f64 * t14726 + 0.10611888591559791_f64 * t14741 + 0.21223777183119583_f64 * t14744 + t10895;
    t19041
}

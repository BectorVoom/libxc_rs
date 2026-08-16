//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 689/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk689(t143: f64, t1556: f64, t169: f64, t1733: f64, t1735: f64, t2211: f64, t242: f64, t2645: f64, t2809: f64, t2880: f64, t2883: f64, t2887: f64, t2893: f64, t2897: f64, t296: f64, t405: f64, t4425: f64, t4427: f64, t4430: f64, t4435: f64, t4439: f64, t5760: f64, t5775: f64, t5777: f64, t5783: f64, t5925: f64, t6016: f64, t6019: f64, t6025: f64, t6037: f64, t6040: f64, t6083: f64, t6087: f64, t6089: f64, t6094: f64, t6098: f64, t6121: f64, t777: f64) -> f64 {
    let t6125 = -3.0_f64 * t5783 * t6016 + 3.0_f64 * t2211 * t6019 - 6.0_f64 * t5783 * t4430 + 12.0_f64 * t6025 * t5925 - t4425 - 0.0011622696607154768_f64 * t4427 + 0.39633663517353707_f64 * t4435 + 0.002711962541669446_f64 * t4439 + (t2880 - 0.14149184788746388_f64 * t2883 - t2887 - 0.28298369577492777_f64 * t5760 + t5775 + 0.21223777183119583_f64 * t5777 + 0.10611888591559791_f64 * t2893 + t2897 + 0.053059442957798957_f64 * t6037 - 0.031835665774679375_f64 * t169 * t6040 * t242 + t6083) * t296 - t777 * t6087 + 3.0_f64 * t6089 * t1735 - t2645 * t1556 + 6.0_f64 * t2809 * t6094 + 3.0_f64 * t1733 * t6098 + 3.0_f64 * t405 * t143 * t6121;
    t6125
}

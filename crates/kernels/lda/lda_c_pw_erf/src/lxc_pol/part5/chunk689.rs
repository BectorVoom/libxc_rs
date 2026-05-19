//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 689/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk689<F: Float>(t143: F, t1556: F, t169: F, t1733: F, t1735: F, t2211: F, t242: F, t2645: F, t2809: F, t2880: F, t2883: F, t2887: F, t2893: F, t2897: F, t296: F, t405: F, t4425: F, t4427: F, t4430: F, t4435: F, t4439: F, t5760: F, t5775: F, t5777: F, t5783: F, t5925: F, t6016: F, t6019: F, t6025: F, t6037: F, t6040: F, t6083: F, t6087: F, t6089: F, t6094: F, t6098: F, t6121: F, t777: F) -> F {
    let t6125 = -F::new(3.0) * t5783 * t6016 + F::new(3.0) * t2211 * t6019 - F::new(6.0) * t5783 * t4430 + F::new(12.0) * t6025 * t5925 - t4425 - F::cast_from(0.0011622696607154768_f64) * t4427 + F::cast_from(0.39633663517353707_f64) * t4435 + F::cast_from(0.002711962541669446_f64) * t4439 + (t2880 - F::cast_from(0.14149184788746388_f64) * t2883 - t2887 - F::cast_from(0.28298369577492777_f64) * t5760 + t5775 + F::cast_from(0.21223777183119583_f64) * t5777 + F::cast_from(0.10611888591559791_f64) * t2893 + t2897 + F::cast_from(0.053059442957798957_f64) * t6037 - F::cast_from(0.031835665774679375_f64) * t169 * t6040 * t242 + t6083) * t296 - t777 * t6087 + F::new(3.0) * t6089 * t1735 - t2645 * t1556 + F::new(6.0) * t2809 * t6094 + F::new(3.0) * t1733 * t6098 + F::new(3.0) * t405 * t143 * t6121;
    t6125
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 522/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk522<F: Float>(t1166: F, t1169: F, t125: F, t1550: F, t1556: F, t169: F, t1733: F, t1735: F, t1808: F, t1809: F, t1881: F, t1885: F, t1891: F, t1898: F, t2203: F, t2205: F, t2208: F, t2211: F, t299: F, t301: F, t405: F, t411: F, t456: F, t777: F) -> F {
    let t2215 = -t777 * t1556 + t777 * t1550 + F::cast_from(6.0_f64) * t1808 * t1809 * t411 + t1881 * t456 - F::cast_from(0.054045904796391424_f64) * t1885 + F::cast_from(0.020267214298646783_f64) * t169 * t299 * t1891 * t301 - F::cast_from(0.0002905674151788692_f64) * t1898 + t2203 * t125 + F::cast_from(3.0_f64) * t405 * t2205 + F::cast_from(3.0_f64) * t1733 * t2208 + F::cast_from(3.0_f64) * t2211 * t1735 + F::cast_from(0.019957056683757683_f64) * t1166 + t1169;
    t2215
}

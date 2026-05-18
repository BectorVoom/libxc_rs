//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1157/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1157<F: Float>(t4602: F, t5474: F, t176: F, t1826: F, t3194: F, t493: F, t10316: F, t10319: F, t10321: F, t10333: F, t13799: F, t13801: F, t13803: F, t13806: F, t13808: F, t13810: F) -> (F, F, F, F, F, F, F) {
    let t13812 = F::new(4.0) / F::new(9.0) * t4602 * t5474;
    let t13816 = F::new(2.0) / F::new(15.0) * t493 * t3194 * t176 * t1826;
    let t13817 = F::new(4.0) / F::new(135.0) * t10316;
    let t13818 = F::new(4.0) / F::new(135.0) * t10319;
    let t13819 = F::new(8.0) / F::new(405.0) * t10321;
    let t13820 = F::new(2.0) / F::new(15.0) * t10333;
    let t13821 = t13799 - t13801 + t13803 + t13806 + t13808 + t13810 - t13812 - t13816 - t13817 - t13818 + t13819 + t13820;
    (t13812, t13816, t13817, t13818, t13819, t13820, t13821)
}

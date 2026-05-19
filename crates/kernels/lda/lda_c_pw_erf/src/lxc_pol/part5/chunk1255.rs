//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1255/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1255<F: Float>(t331: F, t7415: F, t7423: F, t10090: F, t13661: F, t13663: F, t16432: F, t16434: F, t16439: F, t16441: F, t16445: F, t16468: F, t16470: F, t21839: F, t21841: F, t21843: F, t21845: F, t21849: F, t21852: F, t21855: F, t21858: F) -> F {
    let t22500 = t331 * t7415;
    let t22502 = t331 * t7423;
    let t22522 = -F::cast_from(0.008888888888888889_f64) * t22500 + F::cast_from(0.02666666666666667_f64) * t22502 - F::cast_from(0.047988888888888886_f64) * t21839 + F::cast_from(0.07198333333333333_f64) * t21841 + F::cast_from(0.013330246913580247_f64) * t21843 + F::cast_from(0.011997222222222222_f64) * t21845 - F::cast_from(0.035991666666666665_f64) * t21849 + F::new(0.4319) * t21852 - F::cast_from(0.11997222222222222_f64) * t21855 - F::new(0.64785) * t21858 + F::cast_from(0.03732469135802469_f64) * t10090 + F::cast_from(0.044444444444444446_f64) * t13661 - F::cast_from(0.007407407407407408_f64) * t13663 + F::new(0.21595) * t16432 + F::cast_from(0.2879333333333333_f64) * t16434 - F::cast_from(0.02666666666666667_f64) * t16439 + F::cast_from(0.005925925925925926_f64) * t16441 - F::cast_from(0.017777777777777778_f64) * t16445 - F::cast_from(0.02666666666666667_f64) * t16468 + F::cast_from(0.0044444444444444444_f64) * t16470;
    t22522
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1255/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1255(t331: f64, t7415: f64, t7423: f64, t10090: f64, t13661: f64, t13663: f64, t16432: f64, t16434: f64, t16439: f64, t16441: f64, t16445: f64, t16468: f64, t16470: f64, t21839: f64, t21841: f64, t21843: f64, t21845: f64, t21849: f64, t21852: f64, t21855: f64, t21858: f64) -> f64 {
    let t22500 = t331 * t7415;
    let t22502 = t331 * t7423;
    let t22522 = -0.008888888888888889_f64 * t22500 + 0.02666666666666667_f64 * t22502 - 0.047988888888888886_f64 * t21839 + 0.07198333333333333_f64 * t21841 + 0.013330246913580247_f64 * t21843 + 0.011997222222222222_f64 * t21845 - 0.035991666666666665_f64 * t21849 + 0.4319_f64 * t21852 - 0.11997222222222222_f64 * t21855 - 0.64785_f64 * t21858 + 0.03732469135802469_f64 * t10090 + 0.044444444444444446_f64 * t13661 - 0.007407407407407408_f64 * t13663 + 0.21595_f64 * t16432 + 0.2879333333333333_f64 * t16434 - 0.02666666666666667_f64 * t16439 + 0.005925925925925926_f64 * t16441 - 0.017777777777777778_f64 * t16445 - 0.02666666666666667_f64 * t16468 + 0.0044444444444444444_f64 * t16470;
    t22522
}

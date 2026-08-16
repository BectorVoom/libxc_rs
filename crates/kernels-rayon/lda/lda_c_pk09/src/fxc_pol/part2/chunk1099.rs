//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1099/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1099(t12099: f64, t12113: f64, t132: f64, t11461: f64, t11951: f64, t12041: f64, t12075: f64, t12082: f64, t1783: f64, t1856: f64, t2007: f64, t2752: f64, t455: f64, t6928: f64, t6930: f64, t6954: f64, t6956: f64, t6958: f64, t6962: f64, t6964: f64, t6969: f64, t7335: f64, t7337: f64, t7469: f64, t93: f64) -> (f64, f64) {
    let t12114 = t12099 + t12113;
    let t12115 = t132 * t12114;
    let t12119 = t6928 + t6930 - t6954 + t6956 - t6958 - t6962 - 1.2536914064583544_f64 * t6964 + t6969 - t2007 * t11951 - 2.2140749178833072_f64 * t12041 * t455 - t7335 - t7337 - 2.427516195194328_f64 * t12075 * t455 + 2.427516195194328_f64 * t1856 * t11461 + 2.427516195194328_f64 * t7469 * t2752 - 2.427516195194328_f64 * t12082 * t455 - 1.7770439370459628_f64 * t1783 * t93 * t12115;
    (t12114, t12119)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 839/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk839(t7945: f64, t7946: f64, t7948: f64, t7949: f64, t7973: f64, t7978: f64, t7983: f64, t7985: f64, t107: f64, t110: f64, t122: f64, t199: f64, t202: f64, t2422: f64, t2454: f64, t4181: f64, t4185: f64, t4187: f64, t5518: f64, t5520: f64, t5526: f64, t5529: f64, t6918: f64, t6922: f64, t6942: f64, t6944: f64, t6947: f64, t7375: f64, t7425: f64, t7874: f64, t795: f64, t84: f64, t868: f64) -> (f64, f64) {
    let t7988 = t7945 + t7946 + t7948 + t7949 + t7973 + t7978 + t7983 + t7985;
    let t8011 = 3.9861630686838536_f64 * t5529 - 0.011938374665504766_f64 * t122 * t202 * t7988 + 0.42708890021612717_f64 * t107 * t110 * t7425 + 0.5025769232130264_f64 * t6942 + 0.2512884616065132_f64 * t6944 + 0.2512884616065132_f64 * t6947 - t4181 + t4185 + t4187 - 0.15917832887339686_f64 * t5526 - 1.7083556008645087_f64 * t6918 + 0.05969187332752383_f64 * t6922 - 0.5025769232130264_f64 * t5518 - 0.5025769232130264_f64 * t5520 - 0.0837628205355044_f64 * t7874 * t199 - 0.2512884616065132_f64 * t2454 * t868 - 0.2512884616065132_f64 * t795 * t2422 - 0.0837628205355044_f64 * t84 * t7375;
    (t7988, t8011)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 839/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk839<F: Float>(t7945: F, t7946: F, t7948: F, t7949: F, t7973: F, t7978: F, t7983: F, t7985: F, t107: F, t110: F, t122: F, t199: F, t202: F, t2422: F, t2454: F, t4181: F, t4185: F, t4187: F, t5518: F, t5520: F, t5526: F, t5529: F, t6918: F, t6922: F, t6942: F, t6944: F, t6947: F, t7375: F, t7425: F, t7874: F, t795: F, t84: F, t868: F) -> (F, F) {
    let t7988 = t7945 + t7946 + t7948 + t7949 + t7973 + t7978 + t7983 + t7985;
    let t8011 = F::cast_from(3.9861630686838536_f64) * t5529 - F::cast_from(0.011938374665504766_f64) * t122 * t202 * t7988 + F::cast_from(0.42708890021612717_f64) * t107 * t110 * t7425 + F::cast_from(0.5025769232130264_f64) * t6942 + F::cast_from(0.2512884616065132_f64) * t6944 + F::cast_from(0.2512884616065132_f64) * t6947 - t4181 + t4185 + t4187 - F::cast_from(0.15917832887339686_f64) * t5526 - F::cast_from(1.7083556008645087_f64) * t6918 + F::cast_from(0.05969187332752383_f64) * t6922 - F::cast_from(0.5025769232130264_f64) * t5518 - F::cast_from(0.5025769232130264_f64) * t5520 - F::cast_from(0.0837628205355044_f64) * t7874 * t199 - F::cast_from(0.2512884616065132_f64) * t2454 * t868 - F::cast_from(0.2512884616065132_f64) * t795 * t2422 - F::cast_from(0.0837628205355044_f64) * t84 * t7375;
    (t7988, t8011)
}

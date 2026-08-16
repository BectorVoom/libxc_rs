//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1342/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1342(t11081: f64, t26960: f64, t28097: f64, t1268: f64, t13132: f64, t13150: f64, t15227: f64, t26955: f64, t26966: f64, t28098: f64, t28123: f64, t28146: f64, t3515: f64, t5302: f64, t5310: f64, t5336: f64, t922: f64, t92795: f64, t93023: f64, t95566: f64, t96739: f64, t96745: f64, t96754: f64) -> f64 {
    let t96763 = 0.7722800925925925926e-4_f64 * t26960 * t11081 * t28097;
    let t96776 = 0.12356481481481481482e-2_f64 * t26966 * t28146 - 0.46336805555555555556e-3_f64 * t26960 * t96739 + 0.13901041666666666667e-2_f64 * t26960 * t96745 + 0.18550940104166666667e-3_f64 * t26955 * t96745 - 0.15445601851851851852e-3_f64 * t26960 * t5302 * t28123 * t13150 - 0.36039737654320987655e-3_f64 * t26960 * t15227 * t96754 * t13132 - 0.61782407407407407408e-3_f64 * t92795 * t28098 + t96763 - 0.69505208333333333334e-3_f64 * t26960 * t5310 * t28123 * t13132 + 0.23168402777777777778e-3_f64 * t93023 * t28098 + 0.23168402777777777778e-3_f64 * t26960 * t3515 * t1268 * t5336 * t922 + 0.15476481481481481481e-2_f64 * t95566;
    t96776
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1342/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1342<F: Float>(t11081: F, t26960: F, t28097: F, t1268: F, t13132: F, t13150: F, t15227: F, t26955: F, t26966: F, t28098: F, t28123: F, t28146: F, t3515: F, t5302: F, t5310: F, t5336: F, t922: F, t92795: F, t93023: F, t95566: F, t96739: F, t96745: F, t96754: F) -> F {
    let t96763 = F::cast_from(0.7722800925925925926e-4_f64) * t26960 * t11081 * t28097;
    let t96776 = F::cast_from(0.12356481481481481482e-2_f64) * t26966 * t28146 - F::cast_from(0.46336805555555555556e-3_f64) * t26960 * t96739 + F::cast_from(0.13901041666666666667e-2_f64) * t26960 * t96745 + F::cast_from(0.18550940104166666667e-3_f64) * t26955 * t96745 - F::cast_from(0.15445601851851851852e-3_f64) * t26960 * t5302 * t28123 * t13150 - F::cast_from(0.36039737654320987655e-3_f64) * t26960 * t15227 * t96754 * t13132 - F::cast_from(0.61782407407407407408e-3_f64) * t92795 * t28098 + t96763 - F::cast_from(0.69505208333333333334e-3_f64) * t26960 * t5310 * t28123 * t13132 + F::cast_from(0.23168402777777777778e-3_f64) * t93023 * t28098 + F::cast_from(0.23168402777777777778e-3_f64) * t26960 * t3515 * t1268 * t5336 * t922 + F::cast_from(0.15476481481481481481e-2_f64) * t95566;
    t96776
}

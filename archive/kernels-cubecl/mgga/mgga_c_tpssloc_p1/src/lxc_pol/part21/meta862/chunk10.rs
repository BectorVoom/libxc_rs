//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3139/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3139<F: Float>(t18321: F, t3435: F, t1174: F, t15390: F, t1653: F, t24705: F, t3447: F, t3472: F, t3478: F, t44478: F, t457: F, t460: F, t4919: F, t52127: F, t52135: F, t52138: F, t52161: F, t52271: F, t64885: F, t64903: F, t64916: F, t64929: F, t64943: F, t974: F) -> F {
    let t64951 = t18321 * t3435;
    let t64966 = F::cast_from(0.18518518518518518518e-3_f64) * t64885 - F::cast_from(0.81481481481481481481e-2_f64) * t18321 * t3478 + F::cast_from(0.18518518518518518518e-3_f64) * t52127 - F::cast_from(0.57613168724279835389e-3_f64) * t52135 + F::cast_from(0.18518518518518518518e-3_f64) * t52138 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t974 * t457 * (t64903 + t64916 + t64929 + t64943) * t460 - F::cast_from(0.54320987654320987654e-2_f64) * t64951 - F::cast_from(0.81481481481481481481e-2_f64) * t18321 * t3472 - F::cast_from(0.74074074074074074072e-3_f64) * t3447 * t15390 * t52161 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t4919 * t24705 * t1653 - F::cast_from(0.14814814814814814814e-2_f64) * t3447 * t15390 * t52271 - F::cast_from(0.6172839506172839506e-3_f64) * t44478;
    t64966
}

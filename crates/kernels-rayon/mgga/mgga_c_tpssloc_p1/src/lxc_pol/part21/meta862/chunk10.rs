//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3139/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3139(t18321: f64, t3435: f64, t1174: f64, t15390: f64, t1653: f64, t24705: f64, t3447: f64, t3472: f64, t3478: f64, t44478: f64, t457: f64, t460: f64, t4919: f64, t52127: f64, t52135: f64, t52138: f64, t52161: f64, t52271: f64, t64885: f64, t64903: f64, t64916: f64, t64929: f64, t64943: f64, t974: f64) -> f64 {
    let t64951 = t18321 * t3435;
    let t64966 = 0.18518518518518518518e-3_f64 * t64885 - 0.81481481481481481481e-2_f64 * t18321 * t3478 + 0.18518518518518518518e-3_f64 * t52127 - 0.57613168724279835389e-3_f64 * t52135 + 0.18518518518518518518e-3_f64 * t52138 - 0.83333333333333333332e-3_f64 * t1174 * t974 * t457 * (t64903 + t64916 + t64929 + t64943) * t460 - 0.54320987654320987654e-2_f64 * t64951 - 0.81481481481481481481e-2_f64 * t18321 * t3472 - 0.74074074074074074072e-3_f64 * t3447 * t15390 * t52161 + 0.55555555555555555554e-3_f64 * t3447 * t4919 * t24705 * t1653 - 0.14814814814814814814e-2_f64 * t3447 * t15390 * t52271 - 0.6172839506172839506e-3_f64 * t44478;
    t64966
}

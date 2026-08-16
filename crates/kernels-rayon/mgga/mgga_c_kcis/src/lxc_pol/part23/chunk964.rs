//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 964/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk964(t15844: f64, t1354: f64, t6114: f64, t2084: f64, t3938: f64, t3919: f64, t6117: f64, t3947: f64, t5613: f64, t11539: f64, t1919: f64, t1911: f64, t3944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17739 = 0.15476481481481481481e-2_f64 * t15844;
    let t17762 = t6114 * t1354;
    let t17765 = t2084 * t3938;
    let t17768 = t6117 * t3919;
    let t17771 = t5613 * t3947;
    let t17772 = t17771 * t1354;
    let t17775 = t6117 * t3938;
    let t17778 = t1919 * t11539;
    let t17779 = t17778 * t3919;
    let t17784 = t1911 * t3944;
    (t17739, t17762, t17765, t17768, t17772, t17775, t17779, t17784)
}

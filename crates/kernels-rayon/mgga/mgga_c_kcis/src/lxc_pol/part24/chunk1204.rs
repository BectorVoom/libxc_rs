//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1204/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1204(t28203: f64, t3489: f64, t15573: f64, t28131: f64, t7788: f64, t96727: f64, t27014: f64, t28214: f64, t95903: f64, t11081: f64, t26960: f64, t28106: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97015 = t28203 * t3489;
    let t97024 = t15573 * t28131;
    let t97026 = 0.23168402777777777778e-3_f64 * t7788 * t97024;
    let t97028 = 0.46336805555555555556e-3_f64 * t7788 * t96727;
    let t97030 = 0.7722800925925925926e-4_f64 * t27014 * t28214;
    let t97031 = 0.15476481481481481481e-2_f64 * t95903;
    let t97051 = 0.7722800925925925926e-4_f64 * t26960 * t11081 * t28106;
    (t97015, t97024, t97026, t97028, t97030, t97031, t97051)
}

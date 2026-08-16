//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1016/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1016(t23006: f64, t23023: f64, t16663: f64, t18037: f64, t18040: f64, t18041: f64, t21848: f64, t21852: f64, t21856: f64, t21861: f64, t21865: f64, t21872: f64, t21874: f64, t21879: f64, t21881: f64, t21884: f64, t21889: f64, t21896: f64, t21900: f64, t21903: f64, t21908: f64, t626: f64) -> (f64, f64) {
    let t23024 = t23006 + t23023;
    let t23030 = -0.15476481481481481481e-2_f64 * t21848 - 0.15476481481481481481e-2_f64 * t21852 - 0.51588271604938271603e-3_f64 * t21856 - 0.30952962962962962962e-2_f64 * t21861 + 0.25794135802469135802e-2_f64 * t21865 - 0.51588271604938271603e-3_f64 * t16663 + 0.19345601851851851852e-2_f64 * t21872 + 0.10317654320987654321e-2_f64 * t21874 + 0.69644166666666666664e-2_f64 * t21879 - 0.15476481481481481481e-2_f64 * t21881 - 0.61905925925925925925e-2_f64 * t21884 + 0.11349419753086419753e-1_f64 * t21889 + t23024 * t626 - t18037 + t18040 + t18041 + 0.92858888888888888888e-2_f64 * t21896 - 0.25794135802469135802e-3_f64 * t21900 + 0.17024129629629629629e-1_f64 * t21903 + 0.38691203703703703703e-3_f64 * t21908;
    (t23024, t23030)
}

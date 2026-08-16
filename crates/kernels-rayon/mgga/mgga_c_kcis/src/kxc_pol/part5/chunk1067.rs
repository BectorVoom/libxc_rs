//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1067/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1067(t1347: f64, t5586: f64, t1563: f64, t6072: f64, t1911: f64, t3918: f64, t16050: f64, t187: f64, t15934: f64, t15988: f64, t16631: f64, t16719: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17889 = t5586 * t1347;
    let t17892 = t6072 * t1563;
    let t17895 = t1911 * t3918;
    let t17905 = 0.2283111111111111111e-1_f64 * t16050;
    let t17942 = t187 * t5586;
    let t17973 = 0.15476481481481481481e-2_f64 * t15934;
    let t17995 = 0.23214722222222222222e-2_f64 * t15988;
    let t18002 = 0.23214722222222222222e-2_f64 * t16631;
    let t18037 = 0.15476481481481481481e-2_f64 * t16719;
    (t17889, t17892, t17895, t17905, t17942, t17973, t17995, t18002, t18037)
}

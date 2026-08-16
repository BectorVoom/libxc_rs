//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1203/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1203(t15573: f64, t28210: f64, t7788: f64, t15216: f64, t28112: f64, t26955: f64, t15220: f64, t26960: f64, t28124: f64, t28117: f64, t28189: f64, t3489: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96957 = 0.23168402777777777778e-3_f64 * t7788 * t15573 * t28210;
    let t96975 = t15216 * t28112;
    let t96977 = 0.10306077835648148148e-4_f64 * t26955 * t96975;
    let t96980 = 0.10297067901234567901e-3_f64 * t26960 * t15220 * t28124;
    let t96993 = 0.15445601851851851852e-3_f64 * t26960 * t15216 * t28117;
    let t97010 = t28189 * t3489;
    (t96957, t96975, t96977, t96980, t96993, t97010)
}

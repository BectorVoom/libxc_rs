//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 842/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk842(t1971: f64, t75098: f64, t14258: f64, t3148: f64, t9221: f64, t69808: f64, t14125: f64, t14131: f64, t9158: f64, t15379: f64, t70337: f64, t69574: f64) -> (f64, f64, f64, f64, f64) {
    let t75099 = t1971 * t75098;
    let t75100 = t14258 * t75099;
    let t75102 = t9221 * t3148;
    let t75103 = t75102 * t69808;
    let t75106 = t14131 * t14125 * t9158;
    let t75108 = t15379 * t70337;
    let t75110 = 0.23948483403727617128e0_f64 * t69574;
    (t75100, t75103, t75106, t75108, t75110)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1203/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1203(t2783: f64, t5766: f64, t1850: f64, t7444: f64, t1095: f64, t1938: f64, t5830: f64, t1070: f64, t5775: f64, t5777: f64, t1893: f64, t1899: f64, t7278: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20902 = 3.0_f64 * t5766 * t2783;
    let t20904 = 3.0_f64 * t1850 * t7444;
    let t20905 = t1938 * t1095;
    let t20908 = t5830 * t1095;
    let t20911 = t1070 * t5775;
    let t20913 = 0.96491876992155210402e2_f64 * t20911 * t5777;
    let t20916 = 0.48245938496077605201e2_f64 * t1899 * t7278 * t1893;
    (t20902, t20904, t20905, t20908, t20913, t20916)
}

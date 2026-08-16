//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 840/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk840(t13962: f64, t3056: f64, t8475: f64, t13966: f64, t2046: f64, t8486: f64, t13839: f64, t1652: f64, t2044: f64, t3076: f64, t15035: f64, t2160: f64, t638: f64) -> (f64, f64, f64, f64) {
    let t75074 = t3056 * t13962 * t8475;
    let t75077 = t2046 * t13966 * t8486;
    let t75081 = t13839 * t2044 * t3076 * t1652;
    let t75084 = t638 * t2160 * t15035;
    (t75074, t75077, t75081, t75084)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 690/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk690(t504: f64, t7190: f64, t14189: f64, t16156: f64, t13966: f64, t2046: f64, t7305: f64, t14199: f64, t13962: f64, t7311: f64, t14185: f64, t2040: f64, t2048: f64, t3056: f64, t4789: f64, t71: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69054 = t504 * t7190;
    let t69057 = t16156 * t14189;
    let t69060 = t2046 * t13966 * t7305;
    let t69064 = t16156 * t14199;
    let t69067 = t2046 * t13962 * t7311;
    let t69069 = t16156 * t14185;
    let t69082 = t3056 * t2048 * t4789 * t71 * t2040;
    (t69054, t69057, t69060, t69064, t69067, t69069, t69082)
}

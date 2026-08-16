//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 691/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk691(t13962: f64, t3056: f64, t7385: f64, t7301: f64, t7305: f64, t34: f64, t79: f64, t34750: f64, t637: f64, t26007: f64, t271: f64, t71: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69085 = t3056 * t13962 * t7385;
    let t69091 = t3056 * t13962 * t7301;
    let t69094 = t3056 * t13962 * t7305;
    let t69097 = 1.0_f64 / t34 / t79;
    let t69101 = t34750 * t637;
    let t69102 = t26007 * t69097 * t271 * t71 * t69101;
    (t69085, t69091, t69094, t69097, t69101, t69102)
}

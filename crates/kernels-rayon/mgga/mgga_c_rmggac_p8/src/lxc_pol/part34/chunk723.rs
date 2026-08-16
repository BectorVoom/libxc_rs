//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 723/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk723(t2046: f64, t2050: f64, t2153: f64, t31: f64, t2039: f64, t2128: f64, t270: f64, t638: f64, t14136: f64, t7292: f64, t13966: f64, t7301: f64) -> (f64, f64, f64, f64) {
    let t70369 = t2046 * t2050 * t2153 * t31;
    let t70373 = t638 * t2039 * t2128 * t270;
    let t70376 = t638 * t7292 * t14136;
    let t70381 = t2046 * t13966 * t7301;
    (t70369, t70373, t70376, t70381)
}

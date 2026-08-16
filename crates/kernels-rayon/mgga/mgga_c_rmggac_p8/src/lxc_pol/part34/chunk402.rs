//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 402/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk402(t2127: f64, t290: f64, t1223: f64, t28: f64, t212: f64, t672: f64, t2084: f64, t271: f64, t2017: f64, t262: f64, t2016: f64, t49: f64, t639: f64) -> (f64, f64, f64, f64, f64) {
    let t7894 = t290 * t2127;
    let t7919 = t1223 * t28;
    let t7920 = t212 * t7919;
    let t7921 = t672 * t7920;
    let t7926 = t2084 * t271;
    let t7932 = t2017 * t262;
    let t7933 = t2016 * t7932;
    let t7934 = t639 * t49;
    (t7894, t7921, t7926, t7933, t7934)
}

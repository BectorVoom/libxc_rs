//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 599/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk599(t262: f64, t558: f64, t3068: f64, t12200: f64, t559: f64, t797: f64, t1986: f64, t3141: f64, t305: f64, t571: f64, t13848: f64, t13850: f64, t2314: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15313 = t262 * t558;
    let t15314 = t3068 * t15313;
    let t15315 = t12200 * t15314;
    let t15317 = t797 * t559;
    let t15318 = t1986 * t15317;
    let t15319 = t3141 * t15318;
    let t15321 = t305 * t571;
    let t15322 = t1986 * t15321;
    let t15323 = t3141 * t15322;
    let t15326 = t2314 * t13848 * t13850;
    (t15313, t15314, t15315, t15318, t15319, t15322, t15323, t15326)
}

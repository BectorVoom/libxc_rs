//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 601/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk601(t13862: f64, t15204: f64, t14032: f64, t2319: f64, t14041: f64, t13888: f64, t2282: f64, t3133: f64, t553: f64, t560: f64, t3157: f64, t5058: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15344 = t13862 * t15204;
    let t15345 = t14032 * t15344;
    let t15347 = t13862 * t2319;
    let t15348 = t14041 * t15347;
    let t15350 = t13888 * t2282;
    let t15351 = t3133 * t15350;
    let t15353 = t13862 * t553;
    let t15354 = t3133 * t15353;
    let t15356 = t13862 * t560;
    let t15357 = t3133 * t15356;
    let t15359 = t5058 * t3157;
    (t15344, t15345, t15347, t15348, t15350, t15351, t15353, t15354, t15356, t15357, t15359)
}

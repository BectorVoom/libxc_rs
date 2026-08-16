//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 591/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk591(t118: f64, t15203: f64, t14011: f64, t14047: f64, t2319: f64, t14052: f64, t13862: f64, t2282: f64, t3120: f64, t553: f64, t2412: f64, t3154: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15204 = t118 * t15203;
    let t15205 = t14011 * t15204;
    let t15206 = t14047 * t15205;
    let t15208 = t14011 * t2319;
    let t15209 = t14052 * t15208;
    let t15211 = t13862 * t2282;
    let t15212 = t3120 * t15211;
    let t15214 = t14011 * t553;
    let t15215 = t3120 * t15214;
    let t15218 = t2412 * t3154;
    (t15204, t15205, t15206, t15208, t15209, t15211, t15212, t15214, t15215, t15218)
}

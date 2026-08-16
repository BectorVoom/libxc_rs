//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 596/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk596(t15252: f64, t515: f64, t7231: f64, t3351: f64, t8975: f64, t3352: f64, t2144: f64, t8946: f64, t1971: f64, t875: f64, t8936: f64, t3154: f64, t8571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15253 = t515 * t15252;
    let t15254 = t7231 * t15253;
    let t15255 = t3351 * t15254;
    let t15257 = t515 * t8975;
    let t15258 = t3352 * t15257;
    let t15259 = t3351 * t15258;
    let t15261 = t2144 * t8946;
    let t15262 = t1971 * t15261;
    let t15263 = t3351 * t15262;
    let t15265 = t875 * t8936;
    let t15266 = t1971 * t15265;
    let t15267 = t3351 * t15266;
    let t15269 = t8571 * t3154;
    (t15254, t15255, t15258, t15259, t15262, t15263, t15266, t15267, t15269)
}

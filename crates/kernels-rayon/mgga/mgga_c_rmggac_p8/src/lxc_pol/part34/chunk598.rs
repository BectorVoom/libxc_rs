//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 598/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk598(t15287: f64, t2145: f64, t262: f64, t551: f64, t3068: f64, t7282: f64, t2411: f64, t3140: f64) -> (f64, f64, f64, f64, f64) {
    let t15288 = t2145 * t15287;
    let t15290 = t262 * t551;
    let t15291 = t3068 * t15290;
    let t15292 = t7282 * t15291;
    let t15296 = t2411 * t3140;
    (t15288, t15290, t15291, t15292, t15296)
}

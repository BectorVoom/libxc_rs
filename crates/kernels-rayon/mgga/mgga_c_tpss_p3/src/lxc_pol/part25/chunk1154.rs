//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1154/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1154(t15518: f64, t15542: f64, t15566: f64, t15802: f64, t15826: f64, t15860: f64, t15880: f64, t15927: f64, t219: f64, t5271: f64, t1148: f64, t5275: f64, t9739: f64, param_beta: f64) -> (f64, f64, f64, f64) {
    let t15930 = t15518 + t15542 + t15566 + t15802 + t15826 + t15860 + t15880 + t15927;
    let t15931 = param_beta * t15930;
    let t15933 = t5271 * t219;
    let t15944 = t9739 * t5275 * t1148;
    (t15930, t15931, t15933, t15944)
}

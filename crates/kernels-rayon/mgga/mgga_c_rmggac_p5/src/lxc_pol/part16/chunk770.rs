//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 770/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk770(t7834: f64, t838: f64, t35583: f64, t793: f64, t35586: f64, t797: f64, t265: f64, t874: f64, t20: f64, t2018: f64, t2021: f64, t4729: f64) -> (f64, f64, f64, f64, f64) {
    let t36274 = t838 * t7834;
    let t36284 = t793 * t35583;
    let t36286 = t797 * t35586;
    let t36292 = t874 * t265;
    let t36330 = t4729 * t20 * t2018 * t2021;
    (t36274, t36284, t36286, t36292, t36330)
}

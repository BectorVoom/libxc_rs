//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 804/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk804(t15899: f64, t15904: f64, t15907: f64, t15910: f64, t15915: f64, t15919: f64, t15922: f64, t15925: f64, t15929: f64, t15934: f64, t15938: f64, t11027: f64, t11638: f64, t11646: f64, t11647: f64, t11648: f64, t15942: f64, t15945: f64, t15948: f64, t15953: f64, t15957: f64, t15961: f64) -> (f64, f64) {
    let t16515 = -2.0_f64 / 27.0_f64 * t15899 + t15904 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t15907 - 2.0_f64 / 27.0_f64 * t15910 - 2.0_f64 / 9.0_f64 * t15915 - 2.0_f64 / 9.0_f64 * t15919 - 2.0_f64 / 3.0_f64 * t15922 + 8.0_f64 / 9.0_f64 * t15925 + t15929 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t15934 + 4.0_f64 / 9.0_f64 * t15938;
    let t16523 = 2.0_f64 / 27.0_f64 * t15942 + 4.0_f64 / 9.0_f64 * t15945 - 10.0_f64 / 81.0_f64 * t15948 - t11638 - t11646 - t11647 + t11648 - 4.0_f64 / 9.0_f64 * t15953 + 4.0_f64 / 27.0_f64 * t15957 - 4.0_f64 / 9.0_f64 * t15961 - t11027;
    (t16515, t16523)
}

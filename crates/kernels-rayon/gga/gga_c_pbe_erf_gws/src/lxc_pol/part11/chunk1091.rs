//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1091/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1091(t1044: f64, t12493: f64, t17331: f64, t639: f64, t184: f64, t199: f64, t3397: f64, t3486: f64, t16782: f64, t40422: f64, t587: f64, t950: f64) -> (f64, f64, f64) {
    let t47515 = 128.0_f64 / 81.0_f64 * t639 * t17331 * t12493 * t1044;
    let t47519 = 8.0_f64 / 5.0_f64 * t3397 * t3486 * t184 * t199;
    let t47523 = 32.0_f64 / 15.0_f64 * t587 * t16782 * t40422 * t950;
    (t47515, t47519, t47523)
}

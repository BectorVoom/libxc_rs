//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 952/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk952(t11056: f64, t819: f64, t3370: f64, t833: f64, t1074: f64, t1299: f64, t1338: f64, t3416: f64, t1096: f64, t6755: f64, t1348: f64, t6767: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11057 = t819 * t11056;
    let t11058 = 11.0_f64 / 9.0_f64 * t11057;
    let t11063 = t3370 * t833;
    let t11066 = t1074 * t1299;
    let t11145 = t1338 * t3416;
    let t11148 = t6755 * t1096;
    let t11157 = t1348 * t3416;
    let t11162 = t6767 * t1096;
    (t11058, t11063, t11066, t11145, t11148, t11157, t11162)
}

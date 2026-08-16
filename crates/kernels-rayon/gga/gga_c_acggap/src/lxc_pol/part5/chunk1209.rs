//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1209/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1209(t157: f64, t506: f64, t929: f64, t1163: f64, t1165: f64, t1539: f64, t20906: f64, t1036: f64, t1772: f64, t368: f64, t398: f64, t864: f64) -> (f64, f64, f64) {
    let t22048 = t506 * t929 * t157;
    let t22068 = t1163 * t1165 * t20906 * t1539;
    let t22080 = t1036 * t398 * t368 * t1772 * t864;
    (t22048, t22068, t22080)
}

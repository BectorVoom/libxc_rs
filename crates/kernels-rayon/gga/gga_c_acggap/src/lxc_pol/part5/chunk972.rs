//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 972/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk972(t3431: f64, t5272: f64, t3409: f64, t5213: f64, t1982: f64, t4254: f64, t1036: f64, t1095: f64, t1524: f64, t398: f64, t864: f64, t1434: f64, t3770: f64) -> (f64, f64, f64, f64, f64) {
    let t15796 = t3431 * t5272;
    let t15807 = t3409 * t5213;
    let t15814 = t4254 * t1982;
    let t15826 = t1036 * t398 * t1095 * t1524 * t864;
    let t15828 = t3770 * t1434;
    (t15796, t15807, t15814, t15826, t15828)
}

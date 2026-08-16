//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1855/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1855(t3106: f64, t3111: f64, t3156: f64, t3172: f64, t3150: f64, t11997: f64, t3144: f64, t3141: f64, t1032: f64, t3043: f64, t1040: f64, t1035: f64, t11239: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12007 = t3106 * t3111;
    let t12009 = t3172 * t3156;
    let t12010 = t3150 * t12009;
    let t12012 = t3144 * t11997;
    let t12013 = t3141 * t12012;
    let t12020 = t3043 * t1032;
    let t12021 = t12020 * t1040;
    let t12046 = t11239 * t1035;
    (t12007, t12009, t12010, t12012, t12013, t12021, t12046)
}

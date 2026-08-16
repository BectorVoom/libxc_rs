//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1146/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1146(t12009: f64, t3150: f64, t11997: f64, t3144: f64, t3141: f64, t11678: f64, t4910: f64, t3117: f64, t1032: f64, t3043: f64, t1040: f64, t1065: f64, t3075: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12010 = t3150 * t12009;
    let t12012 = t3144 * t11997;
    let t12013 = t3141 * t12012;
    let t12016 = t11678 * t4910;
    let t12017 = t3117 * t12016;
    let t12020 = t3043 * t1032;
    let t12021 = t12020 * t1040;
    let t12024 = t1065 * t3075;
    (t12010, t12012, t12013, t12016, t12017, t12020, t12021, t12024)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3111/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3111(t127: f64, t15700: f64, t15702: f64, t4806: f64, t16208: f64, t372: f64, t15666: f64, t3211: f64, t15656: f64, t3215: f64, t1025: f64, t1663: f64, t2434: f64, t371: f64) -> (f64, f64, f64, f64, f64) {
    let t54667 = t15700 * t127 * t4806 * t15702;
    let t54672 = t372 * t16208;
    let t54678 = t3211 * t15666;
    let t54680 = t15656 * t3215;
    let t54687 = t1025 * t371 * t2434 * t1663;
    (t54667, t54672, t54678, t54680, t54687)
}

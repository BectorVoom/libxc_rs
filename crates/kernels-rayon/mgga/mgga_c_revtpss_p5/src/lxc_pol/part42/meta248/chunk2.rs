//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 941/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk941(t1832: f64, t1300: f64, t198: f64, t336: f64, t3801: f64, t6435: f64, t6437: f64, t6441: f64, t6473: f64, t6476: f64, t6542: f64, t6544: f64, t6546: f64, t6550: f64, t6554: f64, t6558: f64, t6748: f64) -> (f64, f64) {
    let t6752 = t1832 * t1832;
    let t6756 = t1300 * t198 * t336 * t6748 - t198 * t336 * t3801 * t6752 - t6435 + t6437 - t6441 + t6473 + t6476 + t6542 + t6544 - t6546 + t6550 - t6554 - t6558;
    (t6752, t6756)
}

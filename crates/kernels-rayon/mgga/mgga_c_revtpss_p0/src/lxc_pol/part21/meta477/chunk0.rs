//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2043/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2043(t15118: f64, t918: f64, t2889: f64, t4614: f64, t1606: f64, t2439: f64, t4580: f64, t689: f64) -> (f64, f64, f64, f64) {
    let t15119 = t15118 * t918;
    let t15121 = t4614 * t2889;
    let t15123 = t2439 * t1606;
    let t15125 = t689 * t4580;
    (t15119, t15121, t15123, t15125)
}

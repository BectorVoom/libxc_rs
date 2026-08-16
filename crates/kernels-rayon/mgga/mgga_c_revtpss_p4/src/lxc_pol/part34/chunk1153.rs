//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1153/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1153(t1949: f64, t231: f64, t6016: f64, t7076: f64, t1558: f64, t1579: f64, t25392: f64, t5977: f64, t2723: f64, t25416: f64, t1955: f64, t6041: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29674 = t1949 * t6016 * t231;
    let t29675 = t7076 * t29674;
    let t29682 = t1579 * t1558 * t231;
    let t29683 = t25392 * t29682;
    let t29689 = t1949 * t5977;
    let t29690 = t29689 * t231;
    let t29691 = t7076 * t29690;
    let t29694 = t29689 * t2723;
    let t29695 = t25416 * t29694;
    let t29698 = t1955 * t6041;
    (t29674, t29675, t29682, t29683, t29690, t29691, t29694, t29695, t29698)
}

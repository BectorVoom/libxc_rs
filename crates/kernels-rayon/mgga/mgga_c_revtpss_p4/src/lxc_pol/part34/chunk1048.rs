//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1048/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1048(t24393: f64, t24406: f64, t1188: f64, t12555: f64, t24375: f64, t1756: f64, t20671: f64, t1745: f64, t6502: f64, t1744: f64, t20618: f64, t1757: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24407 = t24393 + t24406;
    let t24408 = t24407 * t1188;
    let t24411 = t24375 * t12555;
    let t24414 = t20671 * t1756;
    let t24417 = t1745 * t6502;
    let t24420 = t20618 * t1744;
    let t24423 = t1757 * t6534;
    (t24407, t24408, t24411, t24414, t24417, t24420, t24423)
}

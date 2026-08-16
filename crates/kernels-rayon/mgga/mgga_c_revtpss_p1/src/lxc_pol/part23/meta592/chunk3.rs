//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2232/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2232(t23720: f64, t23814: f64, t300: f64, t23812: f64, t23455: f64, t23459: f64, t23562: f64, t23564: f64, t23567: f64, t23570: f64, t23665: f64, t23698: f64, t23769: f64, t23772: f64) -> (f64, f64, f64) {
    let t23816 = t300 * (t23720 + t23814);
    let t23818 = 0.19751673498613801407e-1_f64 * t300 * t23812;
    let t23819 = -t23665 + t23455 - t23698 - t23459 + t23816 - t23570 + t23562 - t23564 + t23567 - t23769 + t23772 + t23818;
    (t23816, t23818, t23819)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2151/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2151(t1583: f64, t2832: f64, t27383: f64, t1940: f64, t1963: f64, t9342: f64, t30: f64, t41154: f64, t2408: f64, t1468: f64, t2394: f64, t1957: f64, t25392: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98779 = t1583 * t2832;
    let t98780 = t27383 * t98779;
    let t98784 = 3.0_f64 * t1940 * t1963 * t9342;
    let t98785 = t41154 * t30;
    let t98786 = t1583 * t2408;
    let t98787 = t98785 * t98786;
    let t98793 = t1468 * t2394;
    let t98799 = t1957 * t25392;
    (t98779, t98780, t98784, t98786, t98787, t98793, t98799)
}

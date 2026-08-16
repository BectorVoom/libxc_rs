//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1917/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1917(t27375: f64, t890: f64, t27383: f64, t1583: f64, t2832: f64, t30: f64, t41154: f64, t2408: f64, t1468: f64, t2394: f64, t14495: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t98767 = t27375 * t890;
    let t98768 = t27383 * t98767;
    let t98779 = t1583 * t2832;
    let t98780 = t27383 * t98779;
    let t98785 = t41154 * t30;
    let t98786 = t1583 * t2408;
    let t98787 = t98785 * t98786;
    let t98793 = t1468 * t2394;
    let t98801 = t14495 * t689;
    (t98767, t98768, t98779, t98780, t98786, t98787, t98793, t98801)
}

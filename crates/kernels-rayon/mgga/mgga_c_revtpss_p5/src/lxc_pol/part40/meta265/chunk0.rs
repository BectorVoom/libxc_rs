//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 989/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk989(t521: f64, t9413: f64, t182: f64, t2490: f64, t2495: f64, t9368: f64, t1340: f64, t2626: f64, t4038: f64, t2491: f64, t745: f64, t1330: f64, t2608: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9415 = 120.0_f64 * t9413 * t521;
    let t9417 = 1.0_f64 / t2490 / t182;
    let t9419 = t9417 * t9368 * t2495;
    let t9421 = 0.10389515463408878255e3_f64 * t1340 * t9419;
    let t9422 = t4038 * t2626;
    let t9425 = t2491 * t9368 * t745;
    let t9427 = 0.35089341735807877242e1_f64 * t1340 * t9425;
    let t9428 = t1330 * t2608;
    (t9415, t9417, t9419, t9421, t9422, t9425, t9427, t9428)
}

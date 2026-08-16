//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1499/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1499(t18742: f64, t2782: f64, t18681: f64, t231: f64, t2783: f64, t18677: f64, t2723: f64, t4503: f64, t10916: f64, t14577: f64, t14581: f64, t14590: f64, t14596: f64, t14603: f64, t14608: f64, t14948: f64, t18727: f64, t18731: f64, t18733: f64, t18739: f64) -> f64 {
    let t18743 = t2782 * t18742;
    let t18746 = t2783 * t18681 * t231;
    let t18747 = t2782 * t18746;
    let t18750 = t4503 * t18677 * t2723;
    let t18751 = t2782 * t18750;
    let t18754 = -t14577 + 0.14634331517634470219e-1_f64 * t14581 - 0.9757440539382783019e-2_f64 * t18727 - 0.9757440539382783019e-2_f64 * t18731 - t14590 - 0.19514881078765566037e-1_f64 * t18733 + 0.11565819519348392139e-2_f64 * t10916 + t14596 + 0.39029762157531132076e-1_f64 * t14603 + 0.54878743191129263322e-2_f64 * t18739 + 0.54878743191129263322e-2_f64 * t18743 + 0.10975748638225852664e-1_f64 * t18747 - 0.10975748638225852664e-1_f64 * t18751 - t14608 + 0.23131639038696784278e-2_f64 * t14948;
    t18754
}

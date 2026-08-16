//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 661/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk661(t136: f64, t854: f64, t221: f64, t775: f64, t2674: f64, t26: f64, t66: f64) -> (f64, f64, f64, f64) {
    let t2675 = t854 * t136;
    let t2677 = t2675 * t221 * t775;
    let t2678 = t2674 * t2677;
    let t2681 = 1.0_f64 / t66 / t26;
    (t2675, t2677, t2678, t2681)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1057/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1057(t117: f64, t32608: f64, t1310: f64, t2322: f64, t32402: f64, t32404: f64, t32410: f64, t32415: f64, t32417: f64, t32419: f64, t32421: f64, t32576: f64, t32580: f64, t4254: f64, t508: f64, t651: f64, t6985: f64, t7378: f64, t8627: f64, t8637: f64) -> (f64, f64) {
    let t32609 = t32608 * t117;
    let t32612 = -t1310 * t8627 - 2.0_f64 * t2322 * t8637 - 2.0_f64 * t32410 * t651 - t32609 * t508 - 2.0_f64 * t4254 * t8637 - 2.0_f64 * t6985 * t7378 - 2.0_f64 * t32402 - 2.0_f64 * t32404 - t32415 - t32417 - t32419 - t32421 - t32576 + t32580;
    (t32609, t32612)
}

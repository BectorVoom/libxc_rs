//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1005/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1005(t1224: f64, t3367: f64, t2251: f64, t1012: f64, t1121: f64, t404: f64) -> (f64, f64, f64) {
    let t3692 = t1224 * t3367;
    let t3693 = t3692 * t2251;
    let t3694 = t1012 * t3693;
    let t3698 = 1.0_f64 / t404 / t1121;
    (t3693, t3694, t3698)
}

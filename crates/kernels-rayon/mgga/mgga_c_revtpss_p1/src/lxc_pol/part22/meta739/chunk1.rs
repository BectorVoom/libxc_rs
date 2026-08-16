//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2802/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2802(t2453: f64, t40798: f64, t10728: f64, t9794: f64, t10886: f64, t40236: f64, t808: f64, t123: f64, t125: f64, t2452: f64, t40633: f64, t810: f64) -> (f64, f64, f64, f64) {
    let t40799 = t2453 * t40798;
    let t40801 = t40799 * t9794 * t10728;
    let t40804 = t10886 * t808 * t40236;
    let t40810 = 0.30119321664969771194e-5_f64 * t123 * t125 * t40633 * t2452 * t810;
    (t40799, t40801, t40804, t40810)
}

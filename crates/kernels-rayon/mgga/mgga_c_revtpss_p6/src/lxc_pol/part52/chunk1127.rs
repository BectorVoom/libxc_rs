//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1127/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1127(t120110: f64, t25304: f64, t8648: f64, t119971: f64, t120106: f64, t32469: f64, t119868: f64, t2453: f64, t8464: f64, t817: f64, t8485: f64, t93341: f64) -> (f64, f64, f64, f64, f64) {
    let t120114 = t25304 * t8648 * t120110;
    let t120117 = t119971 * t8648 * t120110;
    let t120119 = t32469 * t120106;
    let t120120 = 0.7437465841810202164e-4_f64 * t120119;
    let t120132 = t2453 * t8464 * t119868;
    let t120138 = t93341 * t8485 * t817;
    (t120114, t120117, t120120, t120132, t120138)
}

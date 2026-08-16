//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1808/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1808(t25325: f64, t6547: f64, t23185: f64, t25045: f64, t82074: f64, t6562: f64, t6572: f64, t86893: f64, t23171: f64, t23228: f64, t7488: f64, t214: f64, t4265: f64) -> (f64, f64, f64, f64, f64) {
    let t87733 = t6547 * t25325;
    let t87753 = t23185 * t82074 * t25045;
    let t87776 = t6562 * t86893 * t6572;
    let t87779 = t23171 * t23228 * t7488;
    let t87782 = t214 * t4265;
    (t87733, t87753, t87776, t87779, t87782)
}

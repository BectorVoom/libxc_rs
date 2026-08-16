//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1986/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1986(t13053: f64, t17049: f64, t17090: f64, t2053: f64, t24297: f64, t24305: f64, t2597: f64, t2713: f64, t2718: f64, t29056: f64, t29080: f64, t5637: f64, t5658: f64, t7092: f64, t7842: f64, t855: f64, t92938: f64, t99003: f64, t99019: f64) -> f64 {
    let t101797 = 2.0_f64 * t855 * t2718 * t2053 * t17049 + 4.0_f64 * t2597 * t29080 - t2713 * t29056 - 2.0_f64 * t13053 * t7842 - t24297 * t5658 + 0.38381794893125283518e-1_f64 * t99003 + 2.0_f64 * t17090 * t7092 + 2.0_f64 * t24305 * t5637 - t92938 + 2.0_f64 * t24297 * t5637 + 0.16449340668482264365e-1_f64 * t99019;
    t101797
}

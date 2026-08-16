//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2137/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2137(t87535: f64, t13388: f64, t1888: f64, t6646: f64, t13385: f64, t22996: f64, t23185: f64, t4283: f64, t81914: f64, t25300: f64, t81591: f64, t1484: f64, t6552: f64, t6637: f64, t81658: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87536 = 0.38381794893125283518e-1_f64 * t87535;
    let t87538 = t1888 * t6646 * t13388;
    let t87541 = t1888 * t22996 * t13385;
    let t87544 = t23185 * t81914 * t4283;
    let t87545 = 0.16449340668482264365e-1_f64 * t87544;
    let t87546 = t81591 * t25300;
    let t87547 = 0.76763589786250567036e-1_f64 * t87546;
    let t87554 = t6552 * t6637 * t81658 * t1484;
    (t87536, t87538, t87541, t87545, t87547, t87554)
}

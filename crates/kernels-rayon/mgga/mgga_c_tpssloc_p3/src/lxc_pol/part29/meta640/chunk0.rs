//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2105/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2105(t22986: f64, t22996: f64, t25249: f64, t2633: f64, t81602: f64, t252: f64, t4119: f64, t6646: f64, t829: f64, t25160: f64, t814: f64, t22690: f64, t7520: f64, t81573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87124 = t22986 * t22996 * t25249 * t2633;
    let t87127 = 0.12793931631041761173e0_f64 * t81602;
    let t87130 = t252 * t4119;
    let t87133 = t22986 * t6646 * t87130 * t829;
    let t87135 = t814 * t25160;
    let t87140 = t81573 * t22690 * t7520;
    (t87124, t87127, t87130, t87133, t87135, t87140)
}

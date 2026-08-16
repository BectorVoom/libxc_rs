//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2026/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2026(t3684: f64, t39503: f64, t677: f64, t9722: f64, t12110: f64, t9888: f64, t9467: f64, t9919: f64, t2393: f64, t2535: f64, t9882: f64, t2420: f64, t701: f64, t9778: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39505 = 0.12842595503380418954e1_f64 * t3684 * t39503;
    let t39506 = t677 * t9722;
    let t39508 = 0.38527786510141256862e1_f64 * t3684 * t39506;
    let t39512 = t12110 * t9888;
    let t39514 = t12110 * t9467;
    let t39516 = t677 * t9919;
    let t39518 = 0.1301229756036208781e0_f64 * t3684 * t39516;
    let t39519 = t2393 * t2535;
    let t39521 = 0.43374325201206959368e-1_f64 * t3684 * t39519;
    let t39522 = t12110 * t9882;
    let t39529 = 8.0_f64 * t2420 * t9778 * t701;
    (t39505, t39506, t39508, t39512, t39514, t39516, t39518, t39519, t39521, t39522, t39529)
}

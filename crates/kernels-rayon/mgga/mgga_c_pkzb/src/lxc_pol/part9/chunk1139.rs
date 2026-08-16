//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1139/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1139(t16600: f64, t1542: f64, t2605: f64, t1020: f64, t1816: f64, t16613: f64, t16615: f64, t16617: f64, t16619: f64, t16621: f64, t1009: f64, t4803: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19741 = 0.97592231702715658578e-1_f64 * t16600;
    let t19742 = t1542 * t2605;
    let t19743 = 60.0_f64 * t19742;
    let t19744 = t1020 * t1816;
    let t19748 = 240.0_f64 * t16613;
    let t19749 = 0.31168546390226634765e3_f64 * t16615;
    let t19750 = 0.30762056574649219973e4_f64 * t16617;
    let t19751 = 36.0_f64 * t16619;
    let t19752 = 96.0_f64 * t16621;
    let t19754 = t4803 * t1009;
    (t19741, t19743, t19744, t19748, t19749, t19750, t19751, t19752, t19754)
}

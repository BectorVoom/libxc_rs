//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1347/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1347(t2109: f64, t96469: f64, t96425: f64, t26012: f64, t7974: f64, t5415: f64, t55: f64, t2108: f64, t2240: f64, t5392: f64, t1409: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104740 = t2109 * t96469;
    let t104749 = t2109 * t96425;
    let t104787 = t7974 * t26012;
    let t104818 = t5415 * t55;
    let t104907 = t2240 * t5392 * t2108;
    let t104911 = t605 * t1409 * t2108;
    (t104740, t104749, t104787, t104818, t104907, t104911)
}

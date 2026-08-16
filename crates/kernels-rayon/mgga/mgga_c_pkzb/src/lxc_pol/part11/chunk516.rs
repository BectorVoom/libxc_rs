//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 516/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk516(t1020: f64, t51: f64, t2660: f64, t568: f64, t2575: f64, t616: f64, t615: f64, t1701: f64, t1703: f64, t1706: f64, t1728: f64, t1733: f64, t1768: f64, t1770: f64, t2580: f64, t2583: f64, t2587: f64, t2592: f64, t2595: f64, t2598: f64, t2602: f64, t2642: f64, t2645: f64, t2648: f64, t2655: f64, t2658: f64, t580: f64, t590: f64, t612: f64) -> (f64, f64, f64, f64) {
    let t2661 = t51 * t1020;
    let t2663 = t2660 * t2661 * t568;
    let t2666 = t616 * t2575;
    let t2667 = t615 * t2666;
    let t2670 = t1701 + 7.0_f64 / 144.0_f64 * t1703 + 7.0_f64 / 144.0_f64 * t2580 + t1706 * t2583 / 16.0_f64 - t580 * t2587 / 48.0_f64 + 0.42874018118069736972e-3_f64 * t2592 * t2595 + 0.10003937560882938627e-2_f64 * t2598 + 0.85748036236139473944e-3_f64 * t1733 * t2602 - 0.21437009059034868486e-3_f64 * t590 * t2642 - 0.21437009059034868486e-3_f64 * t2645 * t2648 + 0.10003937560882938627e-2_f64 * t1728 + t1768 + 0.40015750243531754508e-2_f64 * t1770 + 0.85748036236139473944e-3_f64 * t1733 * t2655 + 0.40015750243531754507e-2_f64 * t2658 + 0.42874018118069736972e-2_f64 * t612 * t2663 - 0.85748036236139473944e-3_f64 * t612 * t2667;
    (t2661, t2663, t2667, t2670)
}

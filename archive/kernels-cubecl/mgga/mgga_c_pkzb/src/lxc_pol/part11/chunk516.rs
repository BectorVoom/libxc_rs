//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 516/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk516<F: Float>(t1020: F, t51: F, t2660: F, t568: F, t2575: F, t616: F, t615: F, t1701: F, t1703: F, t1706: F, t1728: F, t1733: F, t1768: F, t1770: F, t2580: F, t2583: F, t2587: F, t2592: F, t2595: F, t2598: F, t2602: F, t2642: F, t2645: F, t2648: F, t2655: F, t2658: F, t580: F, t590: F, t612: F) -> (F, F, F, F) {
    let t2661 = t51 * t1020;
    let t2663 = t2660 * t2661 * t568;
    let t2666 = t616 * t2575;
    let t2667 = t615 * t2666;
    let t2670 = t1701 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t1703 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2580 + t1706 * t2583 / F::cast_from(16.0_f64) - t580 * t2587 / F::cast_from(48.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t2592 * t2595 + F::cast_from(0.10003937560882938627e-2_f64) * t2598 + F::cast_from(0.85748036236139473944e-3_f64) * t1733 * t2602 - F::cast_from(0.21437009059034868486e-3_f64) * t590 * t2642 - F::cast_from(0.21437009059034868486e-3_f64) * t2645 * t2648 + F::cast_from(0.10003937560882938627e-2_f64) * t1728 + t1768 + F::cast_from(0.40015750243531754508e-2_f64) * t1770 + F::cast_from(0.85748036236139473944e-3_f64) * t1733 * t2655 + F::cast_from(0.40015750243531754507e-2_f64) * t2658 + F::cast_from(0.42874018118069736972e-2_f64) * t612 * t2663 - F::cast_from(0.85748036236139473944e-3_f64) * t612 * t2667;
    (t2661, t2663, t2667, t2670)
}

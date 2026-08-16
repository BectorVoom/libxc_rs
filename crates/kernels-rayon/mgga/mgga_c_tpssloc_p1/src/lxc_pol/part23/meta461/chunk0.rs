//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1348/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1348(t21303: f64, t49274: f64, t10704: f64, t42028: f64, t76644: f64, t21239: f64, t4488: f64, t959: f64, t5950: f64, t5919: f64, t5943: f64, t10165: f64, t1052: f64, t1634: f64, t1635: f64, t17588: f64, t18074: f64, t21662: f64, t21663: f64, t21677: f64, t21692: f64, t3174: f64, t388: f64, t43604: f64, t4557: f64, t4660: f64, t5848: f64, t5914: f64, t5920: f64, t69871: f64, t70978: f64, t70980: f64) -> (f64, f64, f64, f64, f64) {
    let t76668 = 0.2069040516770936012e4_f64 * t49274 * t21303;
    let t76671 = 0.62071215503128080361e4_f64 * t42028 * t76644 * t10704;
    let t76674 = 0.46785788981077169656e1_f64 * t959 * t4488 * t21239;
    let t76675 = t5950 * t5950;
    let t76684 = t5919 * t5919;
    let t76706 = t5943 * t5943;
    let t76715 = -36.0_f64 * t10165 * t1052 * t5919 * t5943 + 8.0_f64 * t1052 * t1634 * t21662 * t3174 + 6.0_f64 * t1052 * t3174 * t76706 + 24.0_f64 * t1052 * t43604 * t76684 + 6.0_f64 * t388 * t5848 * t5914 - 4.0_f64 * t1635 * t69871 - 4.0_f64 * t1635 * t70978 - 12.0_f64 * t1635 * t70980 + 24.0_f64 * t17588 * t5920 + 12.0_f64 * t18074 * t5920 - 4.0_f64 * t21663 * t4660 - 24.0_f64 * t21677 * t4557 + 24.0_f64 * t21692 * t4557;
    (t76668, t76671, t76674, t76675, t76715)
}

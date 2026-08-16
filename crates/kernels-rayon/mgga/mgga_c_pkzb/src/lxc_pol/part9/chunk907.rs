//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 907/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk907(t6811: f64, t4883: f64, t1009: f64, t1542: f64, t1545: f64, t1548: f64, t1008: f64, t1625: f64, t83: f64, t1020: f64, t135: f64, t1535: f64, t1536: f64, t1816: f64, t2536: f64, t2537: f64, t2718: f64, t4867: f64, t4870: f64, t4876: f64, t4879: f64, t5191: f64, t6758: f64, t6762: f64, t6763: f64, t6800: f64, t6803: f64, t6805: f64, t6806: f64, t6810: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6812 = 8.0_f64 * t6811;
    let t6813 = 80.0_f64 * t4883;
    let t6819 = t1542 * t1009;
    let t6820 = 20.0_f64 * t6819;
    let t6821 = t1545 * t1009;
    let t6822 = 12.0_f64 * t6821;
    let t6823 = t1548 * t1009;
    let t6824 = 32.0_f64 * t6823;
    let t6825 = t1008 * t1625;
    let t6826 = t83 * t6825;
    let t6827 = 6.0_f64 * t1020 * t135 * t6763 + 3.0_f64 * t1020 * t1535 * t5191 - 6.0_f64 * t1535 * t2537 * t6806 + 12.0_f64 * t1536 * t2718 * t6758 - t1816 * t2536 * t2537 + t4867 + t4870 - t4876 - t4879 + t6762 + t6800 - t6803 + t6805 + t6810 + t6812 + t6813 + t6820 + t6822 - t6824 + t6826;
    (t6812, t6813, t6820, t6822, t6824, t6825, t6826, t6827)
}

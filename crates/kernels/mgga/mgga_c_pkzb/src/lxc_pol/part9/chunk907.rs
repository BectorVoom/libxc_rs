//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 907/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk907<F: Float>(t6811: F, t4883: F, t1009: F, t1542: F, t1545: F, t1548: F, t1008: F, t1625: F, t83: F, t1020: F, t135: F, t1535: F, t1536: F, t1816: F, t2536: F, t2537: F, t2718: F, t4867: F, t4870: F, t4876: F, t4879: F, t5191: F, t6758: F, t6762: F, t6763: F, t6800: F, t6803: F, t6805: F, t6806: F, t6810: F) -> (F, F, F, F, F, F, F, F) {
    let t6812 = F::cast_from(8.0_f64) * t6811;
    let t6813 = F::cast_from(80.0_f64) * t4883;
    let t6819 = t1542 * t1009;
    let t6820 = F::cast_from(20.0_f64) * t6819;
    let t6821 = t1545 * t1009;
    let t6822 = F::cast_from(12.0_f64) * t6821;
    let t6823 = t1548 * t1009;
    let t6824 = F::cast_from(32.0_f64) * t6823;
    let t6825 = t1008 * t1625;
    let t6826 = t83 * t6825;
    let t6827 = F::cast_from(6.0_f64) * t1020 * t135 * t6763 + F::cast_from(3.0_f64) * t1020 * t1535 * t5191 - F::cast_from(6.0_f64) * t1535 * t2537 * t6806 + F::cast_from(12.0_f64) * t1536 * t2718 * t6758 - t1816 * t2536 * t2537 + t4867 + t4870 - t4876 - t4879 + t6762 + t6800 - t6803 + t6805 + t6810 + t6812 + t6813 + t6820 + t6822 - t6824 + t6826;
    (t6812, t6813, t6820, t6822, t6824, t6825, t6826, t6827)
}

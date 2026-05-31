//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 769/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk769<F: Float>(t4880: F, t2607: F, t496: F, t4883: F, t1009: F, t1542: F, t1545: F, t1548: F, t1008: F, t1625: F, t83: F, t1721: F, t2639: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6810 = F::cast_from(4.0_f64) * t4880;
    let t6811 = t496 * t2607;
    let t6813 = F::cast_from(80.0_f64) * t4883;
    let t6819 = t1542 * t1009;
    let t6821 = t1545 * t1009;
    let t6823 = t1548 * t1009;
    let t6825 = t1008 * t1625;
    let t6826 = t83 * t6825;
    let t6864 = t2639 * t1721;
    (t6810, t6811, t6813, t6819, t6821, t6823, t6825, t6826, t6864)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1182/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1182<F: Float>(t1009: F, t5142: F, t1639: F, t7035: F, t639: F, t7177: F, t1625: F, t2557: F, t83: F, t1008: F, t5075: F, t46: F, t552: F, t6798: F, t1548: F, t2607: F) -> (F, F, F, F, F, F, F) {
    let t19756 = t5142 * t1009;
    let t19758 = t7035 * t1639;
    let t19770 = t7177 * t639;
    let t19775 = t83 * t2557 * t1625;
    let t19778 = t83 * t1008 * t5075;
    let t19795 = t6798 * t46 * t552;
    let t19797 = t1548 * t2607;
    (t19756, t19758, t19770, t19775, t19778, t19795, t19797)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 732/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk732<F: Float>(t3216: F, t7627: F, t10143: F, t28: F, t1437: F, t1864: F, t1863: F, t1410: F, t2240: F, t12571: F, t6489: F, t33: F, t7440: F) -> (F, F, F, F, F, F, F) {
    let t25840 = t7627 * t3216;
    let t25927 = t10143 * t28;
    let t26012 = t1864 * t1437;
    let t26013 = t1863 * t26012;
    let t26016 = t2240 * t1410;
    let t26051 = t12571 * t6489;
    let t26083 = t33 * t7440;
    (t25840, t25927, t26012, t26013, t26016, t26051, t26083)
}

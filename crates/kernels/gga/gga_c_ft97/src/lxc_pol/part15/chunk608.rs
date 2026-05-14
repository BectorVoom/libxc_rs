//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 608/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk608<F: Float>(t4861: F, t8675: F, t2253: F, t4874: F, t4885: F, t1073: F, t920: F, t1526: F, t4906: F, t9483: F, t10915: F, t240: F, t2917: F, t342: F, t4910: F, t630: F) -> (F, F, F, F, F, F, F, F) {
    let t17573 = t8675 * t4861;
    let t17626 = t2253 * t4874;
    let t17627 = t2253 * t4885;
    let t17630 = t920 * t1073;
    let t17685 = t1526 * t9483 * t4906;
    let t17687 = t10915 * t240;
    let t17694 = t2917 * t240;
    let t17703 = t342 * t630 * t4910;
    (t17573, t17626, t17627, t17630, t17685, t17687, t17694, t17703)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 867/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk867<F: Float>(t2071: F, t397: F, t37458: F, t554: F, t538: F, t1691: F, t2035: F, t8811: F, t135: F, t1696: F, t1681: F, t527: F, t8832: F, t2059: F, t1995: F, t8851: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40053 = t397 * t2071;
    let t40055 = t37458 * t40053 * t554;
    let t40059 = t37458 * t40053 * t538;
    let t40067 = t2035 * t1691;
    let t40068 = t8811 * t40067;
    let t40069 = t1696 * t135;
    let t40078 = t37458 * t1681 * t554 * t538;
    let t40081 = t527 * t8832;
    let t40084 = t37458 * t397 * t2059 * t538;
    let t40087 = t1995 * t8851;
    (t40055, t40059, t40067, t40068, t40069, t40078, t40081, t40084, t40087)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 635/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk635<F: Float>(t1882: F, t4603: F, t4599: F, t1780: F, t971: F, t1851: F, t4551: F, t103: F, t4495: F, t4589: F, t487: F, t4608: F, t8392: F) -> (F, F, F, F, F, F, F) {
    let t15978 = t1882 * t4603;
    let t15980 = t1882 * t4599;
    let t16030 = t1780 * t971;
    let t16034 = t1851 * t4551;
    let t16052 = t103 * t4495;
    let t16076 = t487 * t4589;
    let t16083 = t8392 * t4608;
    (t15978, t15980, t16030, t16034, t16052, t16076, t16083)
}

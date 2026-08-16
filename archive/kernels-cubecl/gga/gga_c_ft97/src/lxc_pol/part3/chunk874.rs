//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 874/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk874<F: Float>(t17576: F, t2266: F, t358: F, t4872: F, t363: F, t8680: F, t1073: F, t3052: F, t4458: F, t643: F, t15752: F, t3621: F) -> (F, F, F, F, F) {
    let t17577 = t2266 * t17576;
    let t17581 = t4872 * t358;
    let t17583 = t8680 * t17581 * t363;
    let t17586 = t2266 * t3052 * t1073;
    let t17590 = t2266 * t4458 * t643;
    let t17593 = t3621 * t15752;
    (t17577, t17583, t17586, t17590, t17593)
}

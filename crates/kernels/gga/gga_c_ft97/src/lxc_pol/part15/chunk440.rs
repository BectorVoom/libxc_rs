//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 440/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk440<F: Float>(t4589: F, t488: F, t83: F, t3238: F, t979: F, t452: F, t942: F, t986: F, t110: F, t4495: F, t920: F, t1903: F, t1902: F, t1910: F, t1909: F, t4458: F, t447: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4590 = t488 * t4589;
    let t4591 = t83 * t4590;
    let t4594 = t3238 * t979;
    let t4595 = t83 * t4594;
    let t4599 = t452 * t986 * t942;
    let t4603 = t452 * t110 * t4495;
    let t4606 = t920 * t942;
    let t4607 = t1903 * t4606;
    let t4608 = t1902 * t4607;
    let t4611 = t920 * t979;
    let t4612 = t1910 * t4611;
    let t4613 = t1909 * t4612;
    let t4617 = t447 * t110 * t4458;
    (t4590, t4591, t4594, t4595, t4599, t4603, t4607, t4608, t4611, t4612, t4613, t4617)
}

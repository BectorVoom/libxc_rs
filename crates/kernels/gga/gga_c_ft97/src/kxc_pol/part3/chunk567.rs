//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 567/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk567<F: Float>(t4589: F, t488: F, t83: F, t3238: F, t979: F, t452: F, t942: F, t986: F, t110: F, t4495: F, t920: F, t1903: F) -> (F, F, F, F, F, F, F, F) {
    let t4590 = t488 * t4589;
    let t4591 = t83 * t4590;
    let t4594 = t3238 * t979;
    let t4595 = t83 * t4594;
    let t4599 = t452 * t986 * t942;
    let t4603 = t452 * t110 * t4495;
    let t4606 = t920 * t942;
    let t4607 = t1903 * t4606;
    (t4590, t4591, t4594, t4595, t4599, t4603, t4606, t4607)
}

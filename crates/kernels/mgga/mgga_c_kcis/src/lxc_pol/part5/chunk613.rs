//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 613/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk613<F: Float>(t1602: F, t4425: F, t1599: F, t1611: F, t25: F, t286: F, t3977: F, t3754: F, t617: F, t491: F, t610: F, t990: F) -> (F, F, F, F, F, F, F) {
    let t4426 = t4425 * t1602;
    let t4427 = t1599 * t4426;
    let t4429 = t25 * t1611;
    let t4430 = t1599 * t4429;
    let t4432 = t286 * t3977;
    let t4433 = t617 * t3754;
    let t4438 = t610 * t491;
    let t4439 = t4438 * t990;
    (t4426, t4427, t4429, t4430, t4432, t4433, t4439)
}

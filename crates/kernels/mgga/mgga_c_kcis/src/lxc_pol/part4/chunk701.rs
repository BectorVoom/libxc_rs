//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 701/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk701<F: Float>(t109: F, t494: F, t209: F, t617: F, t612: F, t1369: F, t25: F, t1602: F, t1599: F, t1611: F, t286: F, t3977: F, t3754: F, t2642: F, t491: F, t610: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4420 = t109 * t494;
    let t4422 = t209 * t4420 * t617;
    let t4424 = t612 * t4422 / 864.0;
    let t4425 = t25 * t1369;
    let t4426 = t4425 * t1602;
    let t4427 = t1599 * t4426;
    let t4429 = t25 * t1611;
    let t4430 = t1599 * t4429;
    let t4432 = t286 * t3977;
    let t4433 = t617 * t3754;
    let t4434 = t4433 * t2642;
    let t4435 = t4432 * t4434;
    let t4438 = t610 * t491;
    (t4422, t4424, t4425, t4426, t4427, t4429, t4430, t4434, t4435, t4438)
}

//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 744/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk744<F: Float>(t3754: F, t617: F, t2642: F, t4432: F, t491: F, t610: F, t990: F) -> (F, F, F) {
    let t4433 = t617 * t3754;
    let t4434 = t4433 * t2642;
    let t4435 = t4432 * t4434;
    let t4438 = t610 * t491;
    let t4439 = t4438 * t990;
    (t4434, t4435, t4439)
}

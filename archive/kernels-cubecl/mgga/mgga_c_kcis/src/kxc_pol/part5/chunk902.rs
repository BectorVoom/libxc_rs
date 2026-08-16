//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 902/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk902<F: Float>(t20: F, t2314: F, t725: F, t2316: F, t2469: F, t2: F, t2456: F, t647: F, t649: F, t691: F, t3: F, t8572: F) -> (F, F, F, F, F) {
    let t8578 = t2314 * t725 * t20;
    let t8581 = t2316 * t2469;
    let t8585 = t647 * t2456 * t2;
    let t8590 = t649 * t691;
    let t8593 = t8572 * t3;
    (t8578, t8581, t8585, t8590, t8593)
}

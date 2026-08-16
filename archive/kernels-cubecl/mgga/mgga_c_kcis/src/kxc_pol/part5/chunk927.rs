//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 927/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk927<F: Float>(t2526: F, t775: F, t2490: F, t752: F, t691: F, t747: F, t138: F, t86: F, t124: F, t2394: F, t2479: F, t66: F) -> (F, F, F, F) {
    let t8947 = t775 * t2526;
    let t8948 = t2490 * t8947;
    let t8949 = t752 * t8948;
    let t8955 = t691 * t747;
    let t8957 = t86 * t8955 * t138;
    let t8959 = t2394 * t124;
    let t8961 = t86 * t8959 * t138;
    let t8963 = t66 * t2479;
    (t8949, t8957, t8961, t8963)
}

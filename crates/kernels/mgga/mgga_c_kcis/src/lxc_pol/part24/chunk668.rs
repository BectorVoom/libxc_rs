//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 668/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk668<F: Float>(t236: F, t8121: F, t251: F, t4863: F, t2537: F, t779: F, t2728: F, t887: F, t2751: F, t2489: F, t747: F, t2492: F, t752: F, t753: F, t124: F, t2491: F, t774: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8122 = t236 * t8121;
    let t8291 = t251 * t4863;
    let t8522 = t779 * t2537;
    let t8525 = t887 * t2728;
    let t8526 = t8525 * t2751;
    let t8531 = t747 * t2489;
    let t8532 = t8531 * t2492;
    let t8533 = t752 * t8532;
    let t8535 = t753 * t753;
    let t8536 = 1.0 / t8535;
    let t8537 = t124 * t8536;
    let t8538 = t2491 * t774;
    (t8122, t8291, t8522, t8525, t8526, t8531, t8533, t8536, t8537, t8538)
}

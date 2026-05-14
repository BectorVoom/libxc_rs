//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 837/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk837<F: Float>(t2751: F, t8525: F, t2489: F, t747: F, t2492: F, t752: F, t753: F, t124: F, t2491: F, t774: F, t62: F, t143: F, t740: F, t647: F, t97: F, t728: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8526 = t8525 * t2751;
    let t8531 = t747 * t2489;
    let t8532 = t8531 * t2492;
    let t8533 = t752 * t8532;
    let t8535 = t753 * t753;
    let t8536 = 1.0 / t8535;
    let t8537 = t124 * t8536;
    let t8538 = t2491 * t774;
    let t8539 = t62 * t8538;
    let t8540 = t8537 * t8539;
    let t8541 = t752 * t8540;
    let t8543 = t143 * t740;
    let t8546 = t647 * t97;
    let t8547 = t8546 * t728;
    (t8526, t8531, t8533, t8536, t8537, t8538, t8541, t8543, t8546, t8547)
}

//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 952/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk952<F: Float>(t9235: F, t9265: F, t165: F, t2531: F, t782: F, t826: F, t2533: F, t2626: F, t781: F, t142: F, t2539: F, t2538: F) -> (F, F, F, F, F) {
    let t9266 = t9235 + t9265;
    let t9267 = t9266 * t165;
    let t9268 = t2531 * t782;
    let t9269 = t9268 * t826;
    let t9270 = F::new(3.0) * t9269;
    let t9271 = t2533 * t2626;
    let t9272 = F::new(3.0) * t9271;
    let t9273 = t781 * t781;
    let t9274 = F::new(1.0) / t9273;
    let t9275 = t142 * t9274;
    let t9276 = t2539 * t826;
    let t9277 = t9275 * t9276;
    let t9278 = F::new(6.0) * t9277;
    let t9279 = t826 * t2626;
    let t9280 = t2538 * t9279;
    (t9267, t9270, t9272, t9278, t9280)
}

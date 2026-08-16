//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1060/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1060<F: Float>(t21239: F, t4271: F, t7706: F, t14496: F, t14497: F, t30153: F, t30875: F, t416: F, t30273: F, t6287: F, t30294: F, t6279: F) -> (F, F, F, F, F) {
    let t31352 = t4271 * t21239 * t7706;
    let t31356 = t14496 * t14497 * t30153;
    let t31379 = t416 * t30875;
    let t31385 = t6287 * t30273;
    let t31388 = t6279 * t30294;
    (t31352, t31356, t31379, t31385, t31388)
}

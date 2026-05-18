//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 591/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk591<F: Float>(t492: F, t8259: F, t500: F, t2275: F, t6382: F, t2271: F, t2279: F, t499: F, t8072: F, t498: F, t4235: F, t4231: F, t8077: F) -> (F, F, F, F, F, F, F) {
    let t8260 = t8259 * t492;
    let t8261 = t8260 * t500;
    let t8263 = t6382 * t2275;
    let t8265 = t2271 * t2279;
    let t8267 = t499 * t8072;
    let t8268 = t498 * t8267;
    let t8269 = t4235 * t8268;
    let t8271 = t4231 * t8077;
    (t8260, t8261, t8263, t8265, t8268, t8269, t8271)
}

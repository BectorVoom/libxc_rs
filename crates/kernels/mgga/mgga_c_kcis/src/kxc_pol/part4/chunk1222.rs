//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1222/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1222<F: Float>(t18171: F, t5441: F, t4439: F, t12140: F, t617: F, t5427: F, t16069: F, t6151: F, t18148: F, t18152: F, t18156: F, t18160: F, t18164: F, t18170: F, t4447: F, t4459: F, t4465: F, t6141: F) -> (F,) {
    let t18172 = t18171 * t5441;
    let t18174 = t4439 * t18172 / 432.0;
    let t18175 = t12140 * t617;
    let t18176 = t18175 * t5427;
    let t18178 = t4439 * t18176 / 648.0;
    let t18179 = t6151 * t16069;
    let t18182 = -t18148 - t6141 * t4459 / 36.0 + t18152 + t6141 * t4465 / 72.0 - t4439 * t18156 / 216.0 + t4439 * t18160 / 144.0 - t18164 / 2592.0 + t6141 * t4447 / 108.0 - t18170 - t18174 + t18178 + t4439 * t18179 / 432.0;
    (t18182,)
}

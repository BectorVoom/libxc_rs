//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 968/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk968<F: Float>(t240: F, t29350: F, t29352: F, t29354: F, t29356: F, t29359: F, t29362: F, t29628: F, t30119: F, t567: F, t564: F, t2360: F, t9295: F) -> (F, F) {
    let t30121 = t240 * t30119 + t29350 - t29352 + t29354 - t29356 - t29359 + t29362 - t29628;
    let t30122 = t567 * t30121;
    let t30123 = t564 * t30122;
    let t30124 = t30123 / F::new(16.0);
    let t30125 = t2360 * t9295;
    (t30124, t30125)
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1100/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1100<F: Float>(t1464: F, t2038: F, t28503: F, t3954: F, t16937: F, t28442: F, t27369: F, t1650: F, t27356: F, t4012: F, t5709: F, t12234: F, t531: F, t3715: F, t1394: F, t16700: F, t27387: F) -> (F, F, F, F, F, F) {
    let t98069 = t1464 * t28503 * t2038 * t3954;
    let t98072 = t16937 * t28442;
    let t98074 = 0.20612155671296296296e-4 * t27369 * t98072;
    let t98081 = t5709 * t27356 * t1650 * t4012;
    let t98084 = t12234 * t531;
    let t98087 = t5709 * t98084 * t1650 * t3715;
    let t98102 = t1394 * t27387 * t16700;
    (t98069, t98072, t98074, t98081, t98087, t98102)
}

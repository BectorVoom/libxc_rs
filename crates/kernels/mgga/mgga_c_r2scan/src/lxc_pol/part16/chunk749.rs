//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 749/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk749<F: Float>(t495: F, t551: F, t6343: F, t574: F, t536: F, t252: F, t255: F, t571: F, t113: F, t1569: F, t2145: F, t774: F) -> (F, F, F, F, F, F) {
    let t6345 = t551 * t6343 * t495;
    let t6346 = t574 * t6345;
    let t6358 = t536 * t536;
    let t6359 = F::new(1.0) / t6358;
    let t6360 = t6359 * t252;
    let t6362 = t571 * t6360 * t255;
    let t6363 = t1569 * t113;
    let t6394 = t2145 * t774;
    (t6346, t6359, t6360, t6362, t6363, t6394)
}

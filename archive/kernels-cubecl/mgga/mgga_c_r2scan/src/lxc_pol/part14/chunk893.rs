//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 893/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk893<F: Float>(t1584: F, t2620: F, t506: F, t529: F, t8048: F, t1567: F, t978: F, t255: F, t571: F, t2086: F, t980: F, t538: F, t7195: F) -> (F, F, F, F, F) {
    let t8189 = F::cast_from(0.23115257973478049502e0_f64) * t1584 * t2620;
    let t8191 = t529 * t506 * t8048;
    let t8196 = t1567 * t978;
    let t8198 = t571 * t8196 * t255;
    let t8201 = t980 * t2086;
    let t8204 = t529 * t538 * t7195;
    (t8189, t8191, t8198, t8201, t8204)
}

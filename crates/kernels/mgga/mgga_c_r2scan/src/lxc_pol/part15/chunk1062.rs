//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1062/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1062<F: Float>(t37599: F, t2150: F, t37470: F, t574: F, t10810: F, t6402: F, t2101: F, t547: F, t2096: F, t265: F, t267: F, t546: F, t6476: F) -> (F, F, F, F, F, F) {
    let t37600 = F::new(0.21476142888649427853e-4) * t37599;
    let t37616 = t574 * t37470 * t2150;
    let t37619 = t574 * t10810 * t6402;
    let t37625 = t547 * t2101;
    let t37628 = t2096 * t265 * t267;
    let t37630 = t546 * t37625 * t37628 * t6476;
    (t37600, t37616, t37619, t37625, t37628, t37630)
}

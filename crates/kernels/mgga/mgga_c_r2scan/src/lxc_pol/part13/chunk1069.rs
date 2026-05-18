//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1069/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1069<F: Float>(t2183: F, t573: F, t10776: F, t10810: F, t2135: F, t261: F, t5142: F, t7628: F, t10894: F, t1624: F, t574: F, t6541: F) -> (F, F, F, F, F) {
    let t37782 = t2183 * t573;
    let t37788 = t10776 * t10810 * t2135;
    let t37797 = t7628 * t261 * t5142;
    let t37809 = t10894 * t1624;
    let t37812 = t574 * t10810 * t6541;
    (t37782, t37788, t37797, t37809, t37812)
}

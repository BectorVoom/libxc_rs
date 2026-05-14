//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 967/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk967<F: Float>(t261: F, t5142: F, t7628: F, t10894: F, t1624: F, t10810: F, t574: F, t6541: F, t120: F, t6517: F, t2225: F, t10734: F, t254: F, t255: F, t6314: F, t6321: F) -> (F, F, F, F, F) {
    let t37797 = t7628 * t261 * t5142;
    let t37809 = t10894 * t1624;
    let t37812 = t574 * t10810 * t6541;
    let t37816 = t120 * t6517;
    let t37817 = t37816 * t2225;
    let t37822 = t254 * t10734 * t6314 * t255 * t6321;
    (t37797, t37809, t37812, t37817, t37822)
}

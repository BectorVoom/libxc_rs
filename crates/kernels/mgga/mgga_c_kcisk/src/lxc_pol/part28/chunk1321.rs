//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1321/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1321<F: Float>(t17078: F, t1763: F, t1772: F, t32941: F, t7218: F, t34108: F, t5074: F, t17353: F, t33031: F, t34038: F, t10879: F, t9664: F, t9935: F, t17182: F, t34147: F, t34077: F) -> (F, F, F, F, F, F, F, F) {
    let t116413 = t17078 * t1763 * t1772;
    let t116416 = t32941 * t7218;
    let t116423 = t5074 * t34108;
    let t116426 = t33031 * t17353 * t34038;
    let t116465 = t9664 * t10879 * t9935;
    let t116474 = t17182 * t34147;
    let t116476 = 0.69444444444444444446e-2 * t9664 * t116474;
    let t116477 = t17182 * t34077;
    (t116413, t116416, t116423, t116426, t116465, t116474, t116476, t116477)
}

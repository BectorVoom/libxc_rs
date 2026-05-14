//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1291/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1291<F: Float>(t1869: F, t4972: F, t61238: F, t9679: F, t11197: F, t1772: F, t2447: F, t17078: F, t1763: F, t32941: F, t7218: F, t34108: F, t5074: F, t17353: F, t33031: F, t34038: F) -> (F, F, F, F, F, F) {
    let t116406 = t1869 * t9679 * t61238 * t4972;
    let t116409 = t11197 * t2447 * t1772;
    let t116413 = t17078 * t1763 * t1772;
    let t116416 = t32941 * t7218;
    let t116423 = t5074 * t34108;
    let t116426 = t33031 * t17353 * t34038;
    (t116406, t116409, t116413, t116416, t116423, t116426)
}

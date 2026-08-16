//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 976/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk976<F: Float>(t10760: F, t8081: F, t6085: F, t261: F, t2730: F, t3304: F, t10743: F, t924: F, t2699: F, t3290: F, t10872: F, t3591: F) -> (F, F, F, F, F, F, F) {
    let t11724 = t10760 * t8081;
    let t11725 = t6085 * t11724;
    let t11727 = t261 * t2730;
    let t11728 = t3304 * t11727;
    let t11730 = t10743 * t924;
    let t11732 = t3290 * t2699;
    let t11734 = t10872 * t3591;
    (t11724, t11725, t11727, t11728, t11730, t11732, t11734)
}

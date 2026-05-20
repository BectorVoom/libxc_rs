//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1873/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1873<F: Float>(t13769: F, t240: F, t2661: F, t7269: F, t13760: F, t25972: F, t5609: F, t7028: F, t9845: F, t1889: F, t94545: F, t13846: F, t13877: F, t7021: F) -> (F, F, F, F, F) {
    let t98152 = t2661 * t7269 * t240 * t13769;
    let t98156 = t25972 * t13760;
    let t98161 = t9845 * t7028 * t5609;
    let t98165 = t94545 * t1889;
    let t98168 = t7021 * t13846 * t13877;
    (t98152, t98156, t98161, t98165, t98168)
}

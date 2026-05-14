//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1186/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1186<F: Float>(t5052: F, t524: F, t525: F, t1541: F, t1598: F, t2215: F, t6422: F, t545: F, t7613: F, t571: F, t572: F, t6311: F, t19790: F, t495: F, t19789: F, t164: F, t277: F, t783: F, t785: F) -> (F, F, F, F, F, F, F) {
    let t22850 = t524 * t525 * t5052;
    let t22856 = t524 * t1598 * t1541;
    let t22863 = t6422 * t2215;
    let t22868 = t545 * t7613;
    let t22947 = t571 * t572 * t6311;
    let t22948 = t19790 * t495;
    let t22950 = t22947 * t19789 * t22948;
    let t22959 = 0.83631605379377467466e1 * t783 * t785 * t164 * t277;
    (t22850, t22856, t22863, t22868, t22947, t22950, t22959)
}

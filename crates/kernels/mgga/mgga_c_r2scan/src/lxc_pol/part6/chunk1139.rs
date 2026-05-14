//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1139/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1139<F: Float>(t2236: F, t6471: F, t1632: F, t549: F, t551: F, t6566: F, t2177: F, t5159: F, t133: F, t255: F, t6053: F, t546: F, t565: F, t2191: F, t6487: F, t2219: F, t6490: F) -> (F, F, F, F, F, F, F) {
    let t20737 = t2236 * t6471;
    let t20741 = t549 * t551 * t1632 * t6566;
    let t20743 = t2177 * t5159;
    let t20746 = t133 * t6053 * t255;
    let t20747 = t546 * t20746;
    let t20750 = t565 * t20746;
    let t20753 = t6487 * t2191;
    let t20755 = t6490 * t2219;
    (t20737, t20741, t20743, t20747, t20750, t20753, t20755)
}

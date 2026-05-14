//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1146/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1146<F: Float>(t2041: F, t9758: F, t12342: F, t2042: F, t2049: F, t2815: F, t33073: F, t33077: F, t33131: F, t33132: F, t33151: F, t33153: F, t33156: F, t33302: F, t5527: F, t5532: F, t5533: F, t802: F, t9772: F) -> (F, F) {
    let t33306 = t9758 * t2041;
    let t33309 = -t12342 * t2815 - t2042 * t33151 - 2.0 * t2049 * t33306 + 4.0 * t33132 * t5532 + 2.0 * t33153 * t5533 + 2.0 * t33156 * t5532 + t33302 * t802 - 2.0 * t5527 * t9772 - t33073 - t33077 + t33131;
    (t33306, t33309)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 776/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk776<F: Float>(t3034: F, t5: F, t736: F, t5307: F, t5321: F, t5327: F, t7685: F, t7688: F, t7689: F, t7691: F, t7694: F, t7699: F, t7701: F, t7707: F, t3128: F, t585: F) -> (F, F) {
    let t8908 = t3034 * t5;
    let t8909 = t8908 * t736;
    let t8912 = t5307 + t5321 + 0.1350520664e0 * t5327 - 0.23392894490538584828e1 * t7685 + t7688 + 0.69263436422725855035e2 * t7689 + 0.34631718211362927518e2 * t7691 - 0.8103123984e0 * t7694 + 0.2701041328e0 * t7699 - 0.54217906501508699211e-2 * t8909 + 24.0 * t7701 - t7707;
    let t8915 = t3128 * t585;
    (t8912, t8915)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1037/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1037<F: Float>(t10855: F, t110: F, t2591: F, t481: F, t560: F, t11747: F, t545: F, t113: F, t2719: F, t494: F, t146: F, t6533: F, t978: F) -> (F, F, F, F, F, F) {
    let t25851 = t10855 * t110;
    let t25962 = t2591 * t481;
    let t25968 = t2591 * t560;
    let t25983 = t545 * t11747;
    let t25997 = t2719 * t494 * t113;
    let t26088 = t146 * t6533 * t978;
    (t25851, t25962, t25968, t25983, t25997, t26088)
}

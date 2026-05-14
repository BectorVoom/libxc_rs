//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 942/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk942<F: Float>(t494: F, t9507: F, t1553: F, t24209: F, t2531: F, t6212: F, t2252: F, t2562: F, t2185: F, t2567: F, t2599: F, t3433: F, t10855: F, t110: F, t2591: F, t481: F) -> (F, F, F, F, F, F, F, F) {
    let t25684 = t9507 * t494;
    let t25697 = t24209 * t1553;
    let t25737 = t6212 * t2531;
    let t25746 = t2562 * t2252;
    let t25813 = t2567 * t2185;
    let t25826 = t3433 * t2599;
    let t25851 = t10855 * t110;
    let t25962 = t2591 * t481;
    (t25684, t25697, t25737, t25746, t25813, t25826, t25851, t25962)
}

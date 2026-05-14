//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1076/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1076<F: Float>(t17320: F, t94833: F, t48044: F, t7943: F, t1555: F, t28644: F, t4189: F, t51125: F, t585: F, t1552: F, t15808: F, t11776: F, t2066: F, t1395: F, t17433: F, t17427: F) -> (F, F, F, F, F, F, F, F) {
    let t97652 = 6.0 * t94833 * t17320;
    let t97654 = 4.0 * t48044 * t7943;
    let t97657 = 4.0 * t4189 * t28644 * t1555;
    let t97661 = t51125 * t585;
    let t97663 = t15808 * t1552;
    let t97665 = t11776 * t2066;
    let t97667 = t1395 * t17433;
    let t97669 = t1395 * t17427;
    (t97652, t97654, t97657, t97661, t97663, t97665, t97667, t97669)
}

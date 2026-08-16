//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2239/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2239<F: Float>(t17659: F, t3117: F, t1041: F, t17187: F, t248: F, t3051: F, t10422: F, t17704: F, t3070: F, t17680: F, t13969: F, t17692: F) -> (F, F, F, F, F) {
    let t61977 = t3117 * t17659;
    let t61981 = t1041 * t248 * t3051 * t17187;
    let t62013 = t3070 * t10422 * t17704;
    let t62032 = t3070 * t10422 * t17680;
    let t62038 = t1041 * t13969 * t17692;
    (t61977, t61981, t62013, t62032, t62038)
}

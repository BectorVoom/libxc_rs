//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 889/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk889<F: Float>(t30866: F, t30874: F, t30878: F, t30893: F, t1494: F, t7329: F, t1498: F, t30716: F, t500: F, t1411: F, t1983: F, t7585: F, t7586: F, t1165: F, t4555: F, t604: F, t7493: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35025 = 0.17149607247227894789e-2 * t30866;
    let t35028 = 0.32012600194825403606e-1 * t30874;
    let t35030 = 0.16006300097412701803e-1 * t30878;
    let t35034 = 0.28582678745379824648e-3 * t30893;
    let t35039 = t7329 * t1494;
    let t35041 = t7329 * t1498;
    let t35043 = t30716 * t500;
    let t35051 = t7585 * t7586 * t1983 * t1411;
    let t35055 = t7493 * t1165 * t604 * t4555;
    (t35025, t35028, t35030, t35034, t35039, t35041, t35043, t35051, t35055)
}

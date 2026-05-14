//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 878/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk878<F: Float>(t1102: F, t3314: F, t3457: F, t2333: F, t481: F, t795: F, t2304: F, t875: F, t3434: F, t3439: F, t106: F, t1550: F, t97: F, t1266: F, t321: F, t502: F, t818: F) -> (F, F, F, F, F, F, F) {
    let t11008 = t1102 * t3314 * t3457;
    let t11010 = t2333 * t481;
    let t11011 = t11010 * t795;
    let t11015 = t2304 * t875;
    let t11017 = t3434 * t11015 * t3439;
    let t11020 = t97 * t106 * t1550;
    let t11031 = t1266 * t321;
    let t11033 = t502 * t818;
    (t11008, t11011, t11015, t11017, t11020, t11031, t11033)
}

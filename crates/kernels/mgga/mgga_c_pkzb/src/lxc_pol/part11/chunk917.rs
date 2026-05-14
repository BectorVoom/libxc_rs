//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 917/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk917<F: Float>(t10769: F, t5758: F, t7357: F, t9148: F, t261: F, t5745: F, t228: F, t1084: F, t3550: F, t1855: F, t1083: F, t9228: F, t1899: F, t1108: F, t3604: F, t1107: F, t9451: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10771 = -t5758 + 0.37083333333333333334e-1 * t7357 - 0.278125e-1 * t9148 + 0.278125e-1 * t10769;
    let t10772 = t10771 * t261;
    let t10777 = -t5745 + 0.71233333333333333332e-1 * t7357 - 0.53424999999999999999e-1 * t9148 + 0.53425e-1 * t10769;
    let t10779 = 0.621814e-1 * t10777 * t228;
    let t10780 = t1084 * t3550;
    let t10782 = 6.0 * t1855 * t10780;
    let t10783 = t9228 * t1083;
    let t10785 = 0.48245938496077605201e2 * t1899 * t10783;
    let t10786 = t1108 * t3604;
    let t10789 = t9451 * t1107;
    (t10771, t10772, t10777, t10779, t10780, t10782, t10783, t10785, t10786, t10789)
}

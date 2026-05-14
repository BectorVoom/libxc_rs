//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 859/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk859<F: Float>(t1424: F, t48032: F, t40332: F, t40336: F, t1457: F, t1572: F, t47026: F, t1429: F, t46868: F, t549: F, t10532: F, t10533: F, t47803: F, t1456: F, t46941: F, t1445: F, t567: F) -> (F, F, F, F, F, F, F, F) {
    let t48034 = 0.39722766613167140743e-1 * t48032 * t1424;
    let t48047 = 0.15337170381568299871e1 * t40332;
    let t48048 = 0.38342925953920749677e0 * t40336;
    let t48050 = t1572 * t1457 * t47026;
    let t48055 = 0.39722766613167140743e-1 * t1429 * t549 * t46868;
    let t48060 = t10532 * t10533 * t47803;
    let t48066 = 0.35750489951850426669e0 * t1456 * t1457 * t46941;
    let t48069 = 0.23005755572352449806e1 * t567 * t1445 * t46941;
    (t48034, t48047, t48048, t48050, t48055, t48060, t48066, t48069)
}

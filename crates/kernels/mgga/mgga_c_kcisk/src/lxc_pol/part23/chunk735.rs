//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 735/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk735<F: Float>(t1596: F, t6591: F, t1557: F, t1598: F, t2332: F, t3808: F, t3810: F, t4324: F, t4347: F, t4519: F, t548: F, t5983: F, t5986: F, t5989: F, t5994: F, t5999: F, t6004: F, t6009: F, t6012: F, t6227: F, t6231: F, t6236: F, t6426: F, t6579: F, t6588: F) -> (F, F) {
    let t6592 = t6591 * t1596;
    let t6601 = -0.30952962962962962963e-2 * t5983 + 0.11607361111111111111e-2 * t5986 - 0.17411041666666666666e-2 * t5989 + 0.34822083333333333332e-2 * t5994 - 0.11607361111111111111e-2 * t5999 - 0.11607361111111111111e-2 * t6004 - 0.38691203703703703703e-3 * t6009 + 0.11607361111111111111e-2 * t6012 - t4519 - 0.30952962962962962963e-2 * t3808 + 0.11607361111111111111e-2 * t3810 - 0.193e0 * t6426 * t1598 - 0.193e0 * t4324 * t2332 - 0.193e0 * t1557 * t6588 + 0.193e0 * t1557 * t6592 + 0.74498e-1 * t4347 * t6592 - 0.11607361111111111111e-2 * t6227 + 0.77382407407407407407e-3 * t6231 - 0.11607361111111111111e-2 * t6236 + t6579 * t548;
    (t6592, t6601)
}

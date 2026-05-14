//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 725/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk725<F: Float>(t776: F, t12235: F, t12271: F, t5006: F, t10399: F, t5486: F, t5497: F, t1775: F, t5507: F, t695: F, t1060: F, t5509: F, t10777: F, t41: F, t10436: F, t7568: F, t11155: F, t11162: F, t1758: F, t1995: F, t4973: F, t4977: F, t525: F, t5449: F, t642: F, t7567: F, t773: F) -> (F, F, F, F, F) {
    let t777 = t776 < -0.66725e-1;
    let t12272 = t12271 * t12235;
    let t12273 = t5006 * t12272;
    let t12276 = t5486 * t10399;
    let t12277 = t5006 * t12276;
    let t12280 = t5497 * t10399;
    let t12281 = t1775 * t12280;
    let t12284 = t5507 * t695;
    let t12285 = t1060 * t5509;
    let t12286 = t12284 * t12285;
    let t12287 = t1775 * t12286;
    let t12290 = t10777 * t41;
    let t12306 = t7568 * t10436;
    let t12313 = piecewise3(t777, 0.0, 10.0 / 9.0 * t525 * t12290 * t642 - 10.0 / 9.0 * t525 * t5449 * t1758 + 40.0 / 27.0 * t525 * t1995 * t4973 - 10.0 / 9.0 * t525 * t1995 * t4977 - 280.0 / 243.0 * t525 * t773 * t11155 + 40.0 / 27.0 * t7567 * t12306 - 10.0 / 27.0 * t525 * t773 * t11162);
    (t12273, t12277, t12281, t12287, t12313)
}

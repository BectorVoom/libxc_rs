//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1424/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1424<F: Float>(t115539: F, t9516: F, t32436: F, t33883: F, t32338: F, t9850: F, t32342: F, t33794: F, t109378: F, t2331: F, t4376: F, t6204: F, t33873: F, t9512: F, t109793: F, t109797: F, t114241: F, t114245: F, t114248: F, t32346: F, t32350: F, t33830: F, t33832: F, t83423: F, t9536: F, t9539: F) -> (F, F) {
    let t115550 = 0.13402777777777777778e-2 * t9516 * t115539;
    let t115555 = 0.11574074074074074074e-2 * t32436 * t33883;
    let t115558 = t9850 * t32338;
    let t115566 = 0.11574074074074074074e-2 * t33794 * t32342;
    let t115569 = t6204 * t109378 * t2331 * t4376;
    let t115578 = 0.34722222222222222222e-2 * t9512 * t33873;
    let t115579 = t115550 - 0.61728395061728395062e-2 * t109793 - t109797 - 0.17361111111111111111e-2 * t33794 * t32346 - t115555 - 0.23148148148148148148e-2 * t33794 * t32350 + 0.92592592592592592593e-2 * t115558 * t9539 - 0.20833333333333333334e-1 * t9536 * t6204 * t33830 * t83423 - t115566 + 0.10416666666666666667e-1 * t9536 * t115569 - 0.20833333333333333334e-1 * t32436 * t33832 - 0.15476481481481481481e-2 * t114241 + 0.46429444444444444444e-2 * t114245 - 0.38691203703703703704e-2 * t114248 + t115578;
    (t115569, t115579)
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1380/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1380<F: Float>(t35026: F, t9532: F, t109669: F, t109683: F, t109701: F, t115384: F, t115393: F, t115404: F, t115426: F, t115430: F, t115433: F, t119088: F, t119091: F, t120051: F, t2740: F, t35025: F, t9528: F, t9536: F) -> (F,) {
    let t120310 = t35026 * t9532;
    let t120316 = -t115384 - 0.44675925925925925926e-3 * t109669 + t115393 - t115404 - 0.15476481481481481481e-2 * t119088 + 0.61905925925925925925e-2 * t119091 + 0.38580246913580246913e-3 * t109683 - 0.77602083333333333335e-3 * t115426 + 0.13888888888888888889e-1 * t35025 * t9528 * t2740 - t115430 - 0.17361111111111111111e-2 * t120310 + 0.92592592592592592592e-2 * t115433 - 0.20833333333333333334e-1 * t9536 * t120051 - 0.25794135802469135802e-3 * t109701;
    (t120316,)
}

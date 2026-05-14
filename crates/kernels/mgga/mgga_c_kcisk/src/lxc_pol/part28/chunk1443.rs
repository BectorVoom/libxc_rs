//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1443/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1443<F: Float>(t122980: F, t2804: F, t34593: F, t9999: F, t117205: F, t118495: F, t118497: F, t122015: F, t122021: F, t122024: F, t122029: F, t122036: F, t20: F, t2454: F, t2801: F, t2807: F, t33183: F, t34580: F, t35469: F, t35476: F, t7573: F, t9721: F, t9728: F, t9733: F, t9995: F) -> (F,) {
    let t123237 = t2804 * t122980;
    let t123249 = t34593 * t9999;
    let t123256 = -0.27777777777777777778e-1 * t34580 * t9995 + 0.20104166666666666667e-2 * t33183 * t35476 - 0.19345601851851851852e-2 * t122015 + 0.17361111111111111111e-2 * t123237 - 0.23214722222222222221e-2 * t122021 + t118495 + 0.61905925925925925925e-2 * t122024 + 0.20635308641975308642e-2 * t122029 + 0.46429444444444444444e-2 * t117205 - t118497 + 0.27777777777777777778e-1 * t2801 * t7573 * t2454 * t20 * t2807 - 0.41270617283950617283e-2 * t122036 + 0.40208333333333333335e-2 * t123249 * t9728 + 0.52083333333333333333e-2 * t9721 * t35469 + 0.52083333333333333333e-2 * t9733 * t35469;
    (t123256,)
}

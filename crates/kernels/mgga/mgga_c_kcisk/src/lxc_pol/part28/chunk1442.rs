//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1442/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1442<F: Float>(t34484: F, t9991: F, t118360: F, t122762: F, t6758: F, t35438: F, t9724: F, t10005: F, t113181: F, t118493: F, t118494: F, t121971: F, t121973: F, t121976: F, t121979: F, t121982: F, t122008: F, t34469: F, t34474: F, t35511: F, t9728: F, t9995: F) -> (F, F) {
    let t123213 = t9991 * t34484;
    let t123220 = t118360 * t6758 * t122762;
    let t123223 = t9724 * t35438;
    let t123231 = 0.17024129629629629629e-1 * t121971 + 0.10317654320987654321e-2 * t121973 - 0.25794135802469135802e-3 * t121976 + 0.34722222222222222223e-2 * t123213 + 0.46429444444444444444e-2 * t121979 + 0.77382407407407407407e-3 * t121982 - 0.27777777777777777778e-1 * t35511 * t9728 + 0.46296296296296296296e-2 * t113181 * t123220 + t118493 + t118494 + 0.19657407407407407407e-1 * t123223 * t9728 + 0.10416666666666666667e-1 * t34474 * t9995 + 0.38691203703703703703e-2 * t122008 - 0.27777777777777777778e-1 * t10005 * t34469;
    (t123220, t123231)
}

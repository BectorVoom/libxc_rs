//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1320/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1320<F: Float>(t11204: F, t112289: F, t112602: F, t112604: F, t116891: F, t117022: F, t117031: F, t117033: F, t117044: F, t117047: F, t117052: F, t2785: F, t32931: F, t33031: F, t33056: F, t34018: F, t34274: F, t4830: F, t7278: F, t9649: F, t9931: F) -> (F,) {
    let t117056 = 0.41666666666666666668e-1 * t33031 * t117022 + 0.16083333333333333334e-1 * t33056 * t117022 - 0.92592592592592592594e-2 * t112289 * t34018 + t117031 + t117033 + 0.55555555555555555558e-1 * t4830 * t34274 * t2785 - 0.10416666666666666667e-1 * t7278 * t32931 * t2785 + 0.27777777777777777779e-1 * t11204 * t9931 * t2785 - 0.44218518518518518517e-2 * t117044 + 0.16581944444444444444e-2 * t117047 + 0.40208333333333333335e-2 * t9649 * t116891 + 0.16581944444444444444e-2 * t117052 + 0.46296296296296296298e-2 * t112602 - 0.34722222222222222223e-2 * t112604;
    (t117056,)
}

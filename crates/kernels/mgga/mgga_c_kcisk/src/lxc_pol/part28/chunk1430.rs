//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1430/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1430<F: Float>(t34389: F, t34412: F, t35431: F, t9736: F, t113124: F, t2364: F, t34427: F, t113123: F, t117621: F, t118003: F, t118206: F, t118210: F, t118212: F, t121645: F, t121662: F, t122427: F, t122694: F, t23220: F, t33196: F, t34400: F, t34560: F, t34561: F, t35476: F, t9733: F, t9740: F, t9995: F) -> (F, F) {
    let t122892 = t34412 * t34389;
    let t122894 = t35431 * t9736;
    let t122899 = t113124 * t2364 * t34427;
    let t122902 = 0.20833333333333333334e-1 * t9740 * t122694 + 0.67013888888888888888e-3 * t33196 * t122427 - 0.23148148148148148148e-2 * t9740 * t34560 * t34561 * t23220 + 0.61905925925925925924e-2 * t121645 + 0.40208333333333333334e-2 * t118003 * t9995 + t118206 + 0.52083333333333333333e-2 * t9733 * t35476 + t118210 - 0.40208333333333333334e-2 * t117621 * t34400 + 0.30864197530864197531e-2 * t122892 - 0.17361111111111111111e-2 * t122894 + 0.23148148148148148148e-2 * t118212 - 0.23214722222222222222e-2 * t121662 - 0.13402777777777777778e-2 * t113123 * t122899;
    (t122899, t122902)
}

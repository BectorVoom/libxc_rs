//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1368/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1368<F: Float>(t112872: F, t113181: F, t116511: F, t116538: F, t116541: F, t118013: F, t118021: F, t118028: F, t118032: F, t118037: F, t118040: F, t33196: F, t33208: F, t33297: F, t34400: F, t34406: F, t34429: F, t9740: F) -> (F,) {
    let t118044 = -0.51588271604938271604e-3 * t116511 - 0.34722222222222222222e-2 * t113181 * t118013 - 0.10416666666666666667e-1 * t33208 * t34400 - 0.20833333333333333334e-1 * t33208 * t34406 + t118021 - 0.10416666666666666667e-1 * t33297 * t34429 - 0.40208333333333333334e-2 * t112872 * t34429 + 0.10416666666666666667e-1 * t9740 * t118028 + t118032 + 0.46429444444444444444e-2 * t116538 - 0.38691203703703703704e-2 * t116541 + 0.60312500000000000001e-2 * t33196 * t118037 + 0.30864197530864197532e-2 * t118040 + 0.10416666666666666667e-1 * t9740 * t118037;
    (t118044,)
}

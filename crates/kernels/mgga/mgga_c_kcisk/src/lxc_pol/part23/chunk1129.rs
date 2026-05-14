//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1129/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1129<F: Float>(t32045: F, t3799: F, t1339: F, t3487: F, t9461: F, t3491: F, t9438: F, t9422: F, t9442: F, t2718: F, t32002: F, t32006: F, t32008: F, t32010: F, t32015: F, t32019: F, t32022: F, t32026: F, t32030: F, t32035: F, t32043: F, t9429: F, t9446: F, t9449: F, t9454: F) -> (F, F, F, F, F, F, F) {
    let t32046 = t32045 * t3799;
    let t32047 = t1339 * t32046;
    let t32049 = t9461 * t3487;
    let t32050 = t1339 * t32049;
    let t32052 = t3491 * t9438;
    let t32055 = t9422 * t9442;
    let t32057 = -0.88437037037037037034e-2 * t32002 - 0.33163888888888888888e-2 * t32006 + 0.26805555555555555556e-2 * t32008 * t32010 - 0.20833333333333333334e-1 * t9446 * t32015 + 0.20833333333333333334e-1 * t32019 * t9429 - 0.55555555555555555558e-1 * t32022 * t9454 + 0.8041666666666666667e-2 * t32026 * t9429 + 0.10416666666666666667e-1 * t9446 * t32030 - 0.20833333333333333334e-1 * t9446 * t32035 - 0.69444444444444444446e-2 * t32019 * t9449 + 0.18518518518518518519e-1 * t32022 * t9449 - 0.23148148148148148148e-2 * t32043 - 0.33163888888888888888e-2 * t32047 + 0.22109259259259259258e-2 * t32050 + 0.55555555555555555558e-1 * t32052 * t2718 - 0.69444444444444444446e-2 * t32055;
    (t32046, t32047, t32049, t32050, t32052, t32055, t32057)
}

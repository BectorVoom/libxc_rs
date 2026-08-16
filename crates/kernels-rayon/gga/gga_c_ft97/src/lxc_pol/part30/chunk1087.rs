//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1087/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1087(t245: f64, t149700: f64, t149738: f64, t149766: f64, t149802: f64, t149843: f64, t149880: f64, t149919: f64, t149959: f64, t149992: f64, t150020: f64, t151065: f64, t151094: f64, t151350: f64, t151380: f64, t151411: f64, t152474: f64, t1577: f64, t18: f64, t21: f64, t33800: f64, t35786: f64, t363: f64, t5: f64, t7565: f64, t920: f64) -> f64 {
    let t246 = 10000000.0_f64 <= t245;
    let t152493 = piecewise3(t246, 0.0_f64, t5 * (t149700 + t149738 + t149766 + t149802 + t149843 + t149880 + t149919 + t149959 + t149992 + t150020 + t151065 + t151094 + t151350 + t151380 + t151411 + t152474) * t21 / 4.0_f64 + t5 * t35786 * t363 / 4.0_f64 + t5 * t33800 * t920 / 4.0_f64 + t5 * t7565 * t18 * t1577 / 2.0_f64);
    t152493
}

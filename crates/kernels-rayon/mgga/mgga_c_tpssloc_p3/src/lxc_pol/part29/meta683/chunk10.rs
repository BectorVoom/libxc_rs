//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2324/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2324(t15572: f64, t24741: f64, t15501: f64, t24727: f64, t3500: f64, t7337: f64, t15478: f64, t15527: f64, t15656: f64, t15714: f64, t24699: f64, t24706: f64, t24815: f64, t27599: f64, t27636: f64, t27637: f64, t3493: f64, t3496: f64, t3511: f64, t3518: f64, t7339: f64, t7345: f64, t8028: f64, t8031: f64, t86354: f64) -> f64 {
    let t95617 = t24741 * t15572 / 1728.0_f64;
    let t95623 = t3500 * t24727 * t15501;
    let t95627 = t3500 * t7337 * t15501;
    let t95633 = 0.20186378047070195428e-3_f64 * t27636 * t27637 * t24815 * t3493 + 0.10093189023535097714e-3_f64 * t8031 * t24699 + 0.80745512188280781712e-3_f64 * t8028 * t24706 + 5.0_f64 / 6912.0_f64 * t24741 * t15714 - t24741 * t15478 / 1152.0_f64 - t95617 + t7339 * t15527 / 1536.0_f64 - t27599 * t3496 / 288.0_f64 - t95623 * t3511 / 144.0_f64 + t95627 * t3518 / 288.0_f64 - t86354 / 1728.0_f64 + 5.0_f64 / 1152.0_f64 * t7345 * t15656;
    t95633
}

//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 834/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk834(t12809: f64, t190: f64, t214: f64, t12694: f64, t12701: f64, t12703: f64, t12706: f64, t12708: f64, t12710: f64, t12714: f64, t12717: f64, t12771: f64, t12774: f64, t12776: f64, t12779: f64, t12782: f64) -> (f64, f64) {
    let t12810 = t12809 * t190;
    let t12811 = t12810 * t214;
    let t12813 = 3.0_f64 / 4.0_f64 * t12694 + 3.0_f64 / 32.0_f64 * t12701 - 9.0_f64 / 4.0_f64 * t12703 + 3.0_f64 / 64.0_f64 * t12706 - 3.0_f64 / 8.0_f64 * t12708 - 3.0_f64 / 8.0_f64 * t12710 - 3.0_f64 / 4.0_f64 * t12714 + 3.0_f64 / 8.0_f64 * t12717 + t12771 / 64.0_f64 - 3.0_f64 / 16.0_f64 * t12774 + 3.0_f64 / 64.0_f64 * t12776 - t12779 / 8.0_f64 + 3.0_f64 / 4.0_f64 * t12782 - t12811 / 64.0_f64;
    (t12811, t12813)
}

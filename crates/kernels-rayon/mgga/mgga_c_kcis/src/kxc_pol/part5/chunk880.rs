//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 880/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk880(t584: f64, t7257: f64, t583: f64, t1546: f64, t7276: f64, t7278: f64, t7280: f64, t7284: f64, t7288: f64, t7290: f64, t7292: f64, t7294: f64, t7297: f64, t7300: f64, t7302: f64) -> (f64, f64, f64) {
    let t7304 = t584 * t7257;
    let t7305 = t583 * t7304;
    let t7306 = t1546 * t7305;
    let t7308 = -t7276 / 576.0_f64 - t7278 / 3.0_f64 + t7280 / 12.0_f64 - t7284 / 16.0_f64 - t7288 / 192.0_f64 + t7290 / 24.0_f64 - t7292 / 96.0_f64 + t7294 / 128.0_f64 - t7297 / 24.0_f64 + t7300 / 96.0_f64 - t7302 / 8.0_f64 + t7306 / 256.0_f64;
    (t7305, t7306, t7308)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 825/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk825(t8706: f64, t8710: f64, t8712: f64, t8714: f64, t8716: f64, t8718: f64, t8722: f64, t7429: f64, t7435: f64, t7442: f64, t7449: f64, t8171: f64, t8704: f64, t8708: f64, t8720: f64) -> f64 {
    let t9261 = 0.17149607247227894789e-2_f64 * t8706;
    let t9263 = 0.34299214494455789578e-2_f64 * t8710;
    let t9264 = 0.80031500487063509015e-2_f64 * t8712;
    let t9265 = 0.80031500487063509015e-2_f64 * t8714;
    let t9266 = 0.16006300097412701803e-1_f64 * t8716;
    let t9267 = 0.34299214494455789578e-2_f64 * t8718;
    let t9269 = 0.12862205435420921092e-2_f64 * t8722;
    let t9271 = 0.68598428988911579156e-2_f64 * t8704 - t9261 - 0.68598428988911579156e-2_f64 * t8708 + t9263 + t9264 - t9265 + t9266 - t9267 - 0.34299214494455789578e-2_f64 * t8720 - t9269 - 0.94344276868812456207e-3_f64 * t7429 - t7435 - t7442 - t7449 - t8171;
    t9271
}

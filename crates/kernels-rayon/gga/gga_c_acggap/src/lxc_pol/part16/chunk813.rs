//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 813/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk813(t8841: f64, t8847: f64, t8851: f64, t8860: f64, t8876: f64, t8879: f64, t8882: f64, t8898: f64, t8945: f64, t8973: f64, t8981: f64, t1713: f64, t469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9313 = 0.17149607247227894789e-2_f64 * t8841;
    let t9316 = 0.17149607247227894789e-2_f64 * t8847;
    let t9318 = 0.21437009059034868486e-3_f64 * t8851;
    let t9320 = 0.14291339372689912324e-3_f64 * t8860;
    let t9328 = t8876 / 32.0_f64;
    let t9329 = t8879 / 96.0_f64;
    let t9331 = 0.5603125e-1_f64 * t8882;
    let t9335 = 0.21437009059034868486e-3_f64 * t8898;
    let t9348 = 7.0_f64 / 144.0_f64 * t8945;
    let t9356 = 0.64311027177104605458e-2_f64 * t8973;
    let t9359 = 0.94344276868812456204e-2_f64 * t8981;
    let t9469 = t469 * t1713;
    (t9313, t9316, t9318, t9320, t9328, t9329, t9331, t9335, t9348, t9356, t9359, t9469)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 780/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk780(t1494: f64, t2041: f64, t1498: f64, t7529: f64, t7531: f64, t7540: f64, t7546: f64, t7550: f64, t7551: f64, t7558: f64, t7571: f64, t7573: f64, t7589: f64, t7602: f64, t7606: f64, t7612: f64) -> f64 {
    let t8754 = t2041 * t1494;
    let t8756 = t2041 * t1498;
    let t8765 = -t8754 / 48.0_f64 - t8756 / 48.0_f64 - 0.20965394859736101378e-3_f64 * t7529 + 0.47172138434406228102e-3_f64 * t7531 + t7540 + t7546 + t7550 - 0.47172138434406228102e-2_f64 * t7551 - t7558 + 0.21437009059034868486e-3_f64 * t7571 + 0.64311027177104605458e-3_f64 * t7573 - 0.7145669686344956162e-4_f64 * t7589 - t7602 - 0.85748036236139473944e-3_f64 * t7606 + t7612;
    t8765
}

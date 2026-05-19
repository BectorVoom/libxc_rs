//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 780/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk780<F: Float>(t1494: F, t2041: F, t1498: F, t7529: F, t7531: F, t7540: F, t7546: F, t7550: F, t7551: F, t7558: F, t7571: F, t7573: F, t7589: F, t7602: F, t7606: F, t7612: F) -> F {
    let t8754 = t2041 * t1494;
    let t8756 = t2041 * t1498;
    let t8765 = -t8754 / F::new(48.0) - t8756 / F::new(48.0) - F::cast_from(0.20965394859736101378e-3_f64) * t7529 + F::cast_from(0.47172138434406228102e-3_f64) * t7531 + t7540 + t7546 + t7550 - F::cast_from(0.47172138434406228102e-2_f64) * t7551 - t7558 + F::cast_from(0.21437009059034868486e-3_f64) * t7571 + F::cast_from(0.64311027177104605458e-3_f64) * t7573 - F::cast_from(0.7145669686344956162e-4_f64) * t7589 - t7602 - F::cast_from(0.85748036236139473944e-3_f64) * t7606 + t7612;
    t8765
}

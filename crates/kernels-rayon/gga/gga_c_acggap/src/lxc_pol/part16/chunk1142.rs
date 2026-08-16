//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1142/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1142(t1998: f64, t5826: f64, t1165: f64, t5651: f64, t604: f64, t8463: f64, t31037: f64, t31039: f64, t35211: f64, t35213: f64, t35228: f64, t35231: f64, t35249: f64, t35251: f64, t37446: f64, t37447: f64, t37449: f64, t39771: f64, t39775: f64, t39779: f64, t39782: f64, t39784: f64) -> f64 {
    let t39786 = t1998 * t5826;
    let t39790 = t8463 * t1165 * t604 * t5651;
    let t39793 = -0.21437009059034868486e-3_f64 * t39771 - 0.21437009059034868486e-3_f64 * t39775 - t35211 - 0.21437009059034868486e-3_f64 * t39779 - 0.14291339372689912324e-3_f64 * t39782 + 0.32012600194825403606e-1_f64 * t39784 - 0.85748036236139473945e-2_f64 * t39786 + t35213 + t35228 + t35231 - t37446 - t37447 + t37449 - 0.47172138434406228102e-2_f64 * t39790 + t35249 - t35251 + t31037 + 0.40015750243531754508e-2_f64 * t31039;
    t39793
}

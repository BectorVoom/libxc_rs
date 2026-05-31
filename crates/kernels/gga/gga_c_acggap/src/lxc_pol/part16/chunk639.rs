//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 639/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk639<F: Float>(t1579: F, t336: F, t513: F, t1524: F, t535: F, t1755: F, t3409: F, t1173: F, t1180: F, t335: F, t3396: F, t3571: F, t3574: F, t3649: F, t367: F, t4901: F, t4906: F, t4910: F, t4918: F, t4926: F, t4928: F, t6140: F, t6145: F, t6148: F, t6153: F, t6158: F, t6161: F, t6164: F, t6167: F) -> (F, F, F) {
    let t6171 = t336 * t1579 * t513;
    let t6175 = t336 * t535 * t1524;
    let t6180 = t3409 * t1755;
    let t6182 = F::cast_from(0.80031500487063509015e-2_f64) * t4901 - t4906 + t4910 + t4918 - F::cast_from(0.85748036236139473944e-3_f64) * t4926 - F::cast_from(0.40015750243531754508e-2_f64) * t4928 - F::cast_from(0.10289764348336736873e-1_f64) * t3396 * t6140 + F::cast_from(0.17149607247227894789e-2_f64) * t6145 - F::cast_from(0.21437009059034868486e-3_f64) * t1180 * t6148 + F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t6153 + F::cast_from(0.21437009059034868486e-3_f64) * t6158 + F::cast_from(0.85748036236139473944e-3_f64) * t1173 * t6161 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t6164 - t335 * t6167 / F::cast_from(48.0_f64) - t367 * t6171 / F::cast_from(48.0_f64) - t367 * t6175 / F::cast_from(48.0_f64) - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t3571 - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t3574 + F::cast_from(0.40015750243531754507e-2_f64) * t6180 + t3649;
    (t6171, t6175, t6182)
}

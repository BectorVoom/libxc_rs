//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 808/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk808(t3793: f64, t3808: f64, t3812: f64, t5222: f64, t5224: f64, t5226: f64, t5229: f64, t5240: f64, t5243: f64, t5558: f64, t5626: f64, t5709: f64, t5766: f64, t5837: f64, t5882: f64, t5943: f64, t5999: f64, t6095: f64, t6133: f64, t6182: f64, t6223: f64, t6268: f64, t6317: f64, t6366: f64, t6371: f64, t6408: f64) -> f64 {
    let t6413 = t5626 + t3808 + 0.85748036236139473944e-3_f64 * t3812 + t5558 + t5837 - 0.80031500487063509016e-2_f64 * t3793 - t5229 + t6095 + t6133 + t5243 - 0.85748036236139473944e-3_f64 * t5226 + t5709 + t5766 + t5943 + 0.85748036236139473944e-3_f64 * t5240 + t5882 - t5222 - t5224 + t5999 + t6408 + t6182 + t6223 + t6268 + t6317 + t6366 - 7.0_f64 / 72.0_f64 * t6371;
    t6413
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 323/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk323(t1138: f64, t1141: f64, t1145: f64, t1150: f64, t1152: f64, t1156: f64, t1168: f64, t1173: f64, t1177: f64, t1180: f64, t1184: f64, t1190: f64, t1195: f64, t1200: f64, t1205: f64, t335: f64, t367: f64, t418: f64) -> f64 {
    let t1208 = 7.0_f64 / 72.0_f64 * t1138 + 7.0_f64 / 144.0_f64 * t1141 - t335 * t1145 / 24.0_f64 + t1150 * t1152 / 16.0_f64 + t367 * t1156 / 48.0_f64 + 0.42874018118069736972e-3_f64 * t1168 + 0.17149607247227894789e-2_f64 * t1173 * t1177 - 0.85748036236139473944e-3_f64 * t1180 * t1184 + 0.85748036236139473944e-3_f64 * t1180 * t1190 - 0.85748036236139473944e-3_f64 * t418 * t1195 + 0.42874018118069736972e-3_f64 * t418 * t1200 - 0.42874018118069736972e-3_f64 * t418 * t1205;
    t1208
}

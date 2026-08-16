//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1216/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1216(t1891: f64, t3237: f64, t5690: f64, t997: f64, t1180: f64, t1181: f64, t13451: f64, t17068: f64, t17070: f64, t17072: f64, t174: f64, t19510: f64, t22193: f64, t22198: f64, t22200: f64, t22202: f64, t22209: f64, t3169: f64, t387: f64, t418: f64, t422: f64, t5862: f64) -> f64 {
    let t22211 = t3237 * t1891;
    let t22213 = t997 * t5690;
    let t22215 = -0.85748036236139473944e-3_f64 * t1180 * t1181 * t5862 * t3169 - 0.85748036236139473944e-3_f64 * t22193 - 0.80031500487063509016e-2_f64 * t17068 - 0.16006300097412701803e-1_f64 * t17070 + 0.32012600194825403606e-1_f64 * t17072 + 0.20007875121765877254e-2_f64 * t22198 - 0.85748036236139473944e-3_f64 * t22200 + 0.20007875121765877254e-2_f64 * t22202 + t13451 - 0.85748036236139473944e-3_f64 * t418 * t422 * t387 * t174 * t19510 + 0.40015750243531754508e-2_f64 * t22209 + 0.80031500487063509014e-2_f64 * t22211 + 0.80031500487063509014e-2_f64 * t22213;
    t22215
}

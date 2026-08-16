//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1229/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1229(t14345: f64, t1817: f64, t1008: f64, t6205: f64, t3431: f64, t6255: f64, t6260: f64, t3382: f64, t5864: f64, t12752: f64, t1750: f64, t1089: f64, t1180: f64, t1181: f64, t1182: f64, t17308: f64, t17310: f64, t1815: f64, t20906: f64, t301: f64, t368: f64, t398: f64, t418: f64, t429: f64, t5784: f64, t5964: f64, t942: f64, t966: f64) -> f64 {
    let t22507 = t14345 * t1817;
    let t22509 = t1008 * t6205;
    let t22511 = t3431 * t6255;
    let t22513 = t3431 * t6260;
    let t22515 = t3382 * t5864;
    let t22522 = t12752 * t1750;
    let t22524 = 0.85748036236139473944e-3_f64 * t942 * t398 * t966 * t1815 - 0.34299214494455789578e-2_f64 * t418 * t1089 * t368 * t5784 * t301 - 7.0_f64 / 36.0_f64 * t17308 - 0.68598428988911579156e-2_f64 * t418 * t1089 * t429 * t5964 + 0.42874018118069736972e-3_f64 * t22507 - 0.34299214494455789578e-2_f64 * t22509 - 0.16006300097412701803e-1_f64 * t22511 - 0.16006300097412701803e-1_f64 * t22513 - 0.85748036236139473944e-3_f64 * t22515 - 0.85748036236139473944e-3_f64 * t1180 * t1181 * t20906 * t1182 - 0.40015750243531754508e-2_f64 * t17310 - 0.16006300097412701803e-1_f64 * t22522;
    t22524
}

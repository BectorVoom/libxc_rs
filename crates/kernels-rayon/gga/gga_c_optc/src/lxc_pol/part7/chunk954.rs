//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 954/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk954(t9103: f64, t9104: f64, t3126: f64, t8974: f64, t9097: f64, t1162: f64, t3103: f64, t3234: f64, t3244: f64, t4435: f64, t4457: f64, t4464: f64, t9059: f64, t9063: f64, t9066: f64, t9070: f64, t9075: f64, t9078: f64, t9081: f64, t9085: f64, t9088: f64, t9093: f64, t9094: f64, t9099: f64, t9102: f64) -> f64 {
    let t9105 = t9103 * t9104;
    let t9108 = t8974 * t3126;
    let t9109 = t9097 * t9108;
    let t9112 = 0.11360101276506094136e1_f64 * t3244 * t9059 - 0.15486228121497046737e2_f64 * t3103 * t9063 + 0.1169609647897054359e2_f64 * t3234 * t9066 + 0.1949349413161757265e2_f64 * t3234 * t9070 + 0.4645868436449114021e2_f64 * t4435 * t9075 + 0.90151304338550081454e-1_f64 * t1162 * t9078 - 0.23181763972770020946e0_f64 * t1162 * t9081 + 0.28977204965962526182e-1_f64 * t9085 + 0.38636273287950034909e-1_f64 * t9088 + t9093 + 0.28977204965962526182e-1_f64 * t1162 * t9094 - 0.13186481011862155443e4_f64 * t4464 * t9099 + 0.56690705297447127569e5_f64 * t9102 * t9105 + 0.26372962023724310886e4_f64 * t4457 * t9109;
    t9112
}

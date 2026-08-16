//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1358/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1358(t27031: f64, t8976: f64, t1111: f64, t3093: f64, t530: f64, t26255: f64, t8425: f64, t22035: f64, t24: f64, t8936: f64, t1: f64, t1122: f64, t11982: f64, t26999: f64, t27002: f64, t27005: f64, t27008: f64, t27011: f64, t27012: f64, t27017: f64, t27023: f64, t27027: f64, t3145: f64, t322: f64, t8966: f64, t8968: f64, t8973: f64) -> (f64, f64) {
    let t27032 = t27031 * t8976;
    let t27035 = t1111 * t530 * t3093;
    let t27037 = t8425 * t26255;
    let t27038 = t27037 * t22035;
    let t27043 = t1111 * t24 * t8936;
    let t27045 = -0.28345352648723563785e5_f64 * t26999 + 0.21464596271083352727e-2_f64 * t27002 + 0.48295341609937543636e-2_f64 * t27005 + 0.47242254414539272975e4_f64 * t27008 - 0.5680050638253047068e0_f64 * t11982 * t27011 * t27012 + 0.36629113921839320676e2_f64 * t8973 * t8968 * t27017 + 0.47333755318775392234e0_f64 * t11982 * t3145 * t1122 * t1 * t27023 - 0.18314556960919660338e2_f64 * t8966 * t8968 * t27027 + 0.48838818562452427568e2_f64 * t27032 + t27035 / 108.0_f64 - 7.0_f64 / 54.0_f64 * t1111 * t322 * t27038 + t27043 / 216.0_f64;
    (t27038, t27045)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1358/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1358<F: Float>(t27031: F, t8976: F, t1111: F, t3093: F, t530: F, t26255: F, t8425: F, t22035: F, t24: F, t8936: F, t1: F, t1122: F, t11982: F, t26999: F, t27002: F, t27005: F, t27008: F, t27011: F, t27012: F, t27017: F, t27023: F, t27027: F, t3145: F, t322: F, t8966: F, t8968: F, t8973: F) -> (F, F) {
    let t27032 = t27031 * t8976;
    let t27035 = t1111 * t530 * t3093;
    let t27037 = t8425 * t26255;
    let t27038 = t27037 * t22035;
    let t27043 = t1111 * t24 * t8936;
    let t27045 = -F::cast_from(0.28345352648723563785e5_f64) * t26999 + F::cast_from(0.21464596271083352727e-2_f64) * t27002 + F::cast_from(0.48295341609937543636e-2_f64) * t27005 + F::cast_from(0.47242254414539272975e4_f64) * t27008 - F::cast_from(0.5680050638253047068e0_f64) * t11982 * t27011 * t27012 + F::cast_from(0.36629113921839320676e2_f64) * t8973 * t8968 * t27017 + F::cast_from(0.47333755318775392234e0_f64) * t11982 * t3145 * t1122 * t1 * t27023 - F::cast_from(0.18314556960919660338e2_f64) * t8966 * t8968 * t27027 + F::cast_from(0.48838818562452427568e2_f64) * t27032 + t27035 / F::cast_from(108.0_f64) - F::cast_from(7.0_f64) / F::cast_from(54.0_f64) * t1111 * t322 * t27038 + t27043 / F::cast_from(216.0_f64);
    (t27038, t27045)
}

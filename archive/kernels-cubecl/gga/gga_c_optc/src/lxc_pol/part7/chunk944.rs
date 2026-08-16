//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 944/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk944<F: Float>(t8527: F, t8979: F, t1146: F, t3160: F, t1141: F, t3169: F, t1145: F, t454: F, t1182: F, t3171: F, t3264: F, t2367: F, t3224: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8980 = t8527 + t8979;
    let t8984 = t3160 * t1146;
    let t8988 = t1141 * t3169;
    let t8995 = t1145 * t1145;
    let t8996 = F::cast_from(1.0_f64) / t8995;
    let t8997 = t454 * t8996;
    let t8998 = t3171 * t1182;
    let t9002 = t3169 * t1182;
    let t9003 = t9002 * t3264;
    let t9006 = t2367 * t3224;
    (t8980, t8984, t8988, t8995, t8996, t8997, t8998, t9002, t9003, t9006)
}

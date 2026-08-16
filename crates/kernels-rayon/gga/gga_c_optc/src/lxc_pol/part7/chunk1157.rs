//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1157/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1157(t7213: f64, t8276: f64, t2433: f64, t2368: f64, t7304: f64, t7198: f64, t984: f64, t7330: f64, t2329: f64, t881: f64, t2364: f64, t24037: f64, t24041: f64, t24044: f64, t24046: f64, t280: f64, t287: f64, t7268: f64, t8: f64, t8291: f64, t8297: f64, t8381: f64, t989: f64) -> f64 {
    let t24049 = t7213 * t8276;
    let t24050 = t2433 * t24049;
    let t24052 = t7304 * t2368;
    let t24054 = t984 * t7198;
    let t24058 = t984 * t7330;
    let t24060 = t2329 * t881;
    let t24068 = -t24037 - 32.0_f64 / 3.0_f64 * t2364 * t7268 - 16.0_f64 / 9.0_f64 * t24041 + t24044 + 400.0_f64 / 27.0_f64 * t2433 * t24046 + 200.0_f64 / 81.0_f64 * t24050 - 32.0_f64 / 9.0_f64 * t24052 + 176.0_f64 / 9.0_f64 * t24054 - 16.0_f64 / 3.0_f64 * t8381 * t989 + 20.0_f64 / 27.0_f64 * t24058 - 392000000.0_f64 / 729.0_f64 * t8291 / t280 / t24060 * t8 * t287 * t8297;
    t24068
}

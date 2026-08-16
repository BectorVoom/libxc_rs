//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1018/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1018(t13287: f64, t13293: f64, t1449: f64, t4210: f64, t13364: f64, t5122: f64, t8401: f64, t13299: f64, t5127: f64, t1095: f64, t3101: f64, t384: f64, t398: f64, t513: f64) -> (f64, f64, f64, f64) {
    let t17258 = t13293 * t13287 * t1449 * t4210;
    let t17262 = t13293 * t13364 * t8401 * t5122;
    let t17266 = t13293 * t13299 * t8401 * t5127;
    let t17281 = t384 * t398 * t1095 * t513 * t3101;
    (t17258, t17262, t17266, t17281)
}

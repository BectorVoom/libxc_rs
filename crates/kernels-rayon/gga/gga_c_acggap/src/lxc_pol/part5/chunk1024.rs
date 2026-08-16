//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1024/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1024(t13298: f64, t13299: f64, t1432: f64, t3196: f64, t1165: f64, t1532: f64, t15758: f64, t3451: f64, t1539: f64, t3194: f64, t839: f64, t1163: f64, t1181: f64, t4289: f64, t5122: f64) -> (f64, f64, f64, f64) {
    let t17430 = t13298 * t13299 * t1432 * t3196;
    let t17436 = t3451 * t1165 * t1532 * t15758;
    let t17441 = t3194 * t1165 * t1532 * t1539 * t839;
    let t17445 = t1163 * t1181 * t4289 * t5122;
    (t17430, t17436, t17441, t17445)
}

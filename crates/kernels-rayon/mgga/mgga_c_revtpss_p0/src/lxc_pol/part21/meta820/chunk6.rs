//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3033/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3033(t15669: f64, t3286: f64, t1651: f64, t378: f64, t11804: f64, t12057: f64, t12149: f64, t12150: f64, t12157: f64, t12167: f64, t12168: f64, t16076: f64, t16433: f64, t16502: f64, t16506: f64, t16534: f64, t3259: f64, t3298: f64, t3322: f64, t342: f64, t43341: f64, t43360: f64, t43378: f64, t4743: f64, t4954: f64, t4976: f64, t4977: f64, t4984: f64, t4996: f64, t4998: f64, t54474: f64, t54909: f64, t55330: f64, t55499: f64, t55586: f64, t73: f64) -> f64 {
    let t55747 = t15669 * t3286;
    let t55764 = t378 * t1651;
    let t55783 = 0.39512695097613069591e1_f64 * t55747 * t12150 + 0.19756347548806534796e1_f64 * t4743 * t3322 + 0.39512695097613069591e1_f64 * t342 * t3298 * t3259 * t4984 - 0.39512695097613069591e1_f64 * t43378 * t4977 - 0.19756347548806534796e1_f64 * t16502 * t12157 - 0.39512695097613069591e1_f64 * t16506 * t16534 + 0.19756347548806534796e1_f64 * t4954 * t12057 - 0.11853808529283920877e2_f64 * t55330 * t55764 * t11804 + 0.39512695097613069591e1_f64 * t12167 * t55586 * t12168 + 0.39512695097613069591e1_f64 * t12149 * t16076 * t73 * t4976 - 0.19756347548806534796e1_f64 * t43341 * t55499 * t54474 - 0.79025390195226139182e1_f64 * t43360 * t16433 - 0.19756347548806534796e1_f64 * t4996 * t54909 * t4998;
    t55783
}

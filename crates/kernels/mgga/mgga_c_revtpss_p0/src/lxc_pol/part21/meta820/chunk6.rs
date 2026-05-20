//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3033/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3033<F: Float>(t15669: F, t3286: F, t1651: F, t378: F, t11804: F, t12057: F, t12149: F, t12150: F, t12157: F, t12167: F, t12168: F, t16076: F, t16433: F, t16502: F, t16506: F, t16534: F, t3259: F, t3298: F, t3322: F, t342: F, t43341: F, t43360: F, t43378: F, t4743: F, t4954: F, t4976: F, t4977: F, t4984: F, t4996: F, t4998: F, t54474: F, t54909: F, t55330: F, t55499: F, t55586: F, t73: F) -> F {
    let t55747 = t15669 * t3286;
    let t55764 = t378 * t1651;
    let t55783 = F::cast_from(0.39512695097613069591e1_f64) * t55747 * t12150 + F::cast_from(0.19756347548806534796e1_f64) * t4743 * t3322 + F::cast_from(0.39512695097613069591e1_f64) * t342 * t3298 * t3259 * t4984 - F::cast_from(0.39512695097613069591e1_f64) * t43378 * t4977 - F::cast_from(0.19756347548806534796e1_f64) * t16502 * t12157 - F::cast_from(0.39512695097613069591e1_f64) * t16506 * t16534 + F::cast_from(0.19756347548806534796e1_f64) * t4954 * t12057 - F::cast_from(0.11853808529283920877e2_f64) * t55330 * t55764 * t11804 + F::cast_from(0.39512695097613069591e1_f64) * t12167 * t55586 * t12168 + F::cast_from(0.39512695097613069591e1_f64) * t12149 * t16076 * t73 * t4976 - F::cast_from(0.19756347548806534796e1_f64) * t43341 * t55499 * t54474 - F::cast_from(0.79025390195226139182e1_f64) * t43360 * t16433 - F::cast_from(0.19756347548806534796e1_f64) * t4996 * t54909 * t4998;
    t55783
}

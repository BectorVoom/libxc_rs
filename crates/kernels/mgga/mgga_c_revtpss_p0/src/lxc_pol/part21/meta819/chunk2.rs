//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3023/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3023<F: Float>(t1000: F, t1076: F, t1079: F, t11122: F, t11123: F, t11195: F, t11201: F, t11202: F, t11207: F, t12040: F, t12174: F, t16287: F, t16305: F, t16328: F, t16362: F, t16374: F, t1695: F, t225: F, t3047: F, t3060: F, t3067: F, t3076: F, t3269: F, t3271: F, t3325: F, t342: F, t385: F, t42060: F, t42067: F, t4747: F, t4752: F, t4778: F, t4935: F, t4947: F, t5015: F, t53223: F, t53273: F, t53281: F, t54983: F, t55377: F, t995: F, t996: F) -> F {
    let t55405 = F::cast_from(0.19756347548806534796e1_f64) * t4747 * t11207 - F::cast_from(0.39512695097613069591e1_f64) * t4778 * t12040 - F::cast_from(0.19756347548806534796e1_f64) * t53223 * t1000 + F::cast_from(0.19756347548806534796e1_f64) * t4778 * t11207 + F::cast_from(0.39512695097613069591e1_f64) * t11195 * t4947 - F::cast_from(0.39512695097613069591e1_f64) * t4935 * t11123 - F::cast_from(0.65854491829355115987e0_f64) * t995 * t996 * t53273 + F::cast_from(0.39512695097613069591e1_f64) * t16362 * t3271 - F::cast_from(0.65854491829355115987e0_f64) * t4752 * t12174 + F::cast_from(0.39512695097613069591e1_f64) * t53281 * t3060 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t55377 * t225 * t385 + F::cast_from(0.39512695097613069591e1_f64) * t11201 * t1079 * t1695 * t11202 + F::cast_from(0.15805078039045227836e2_f64) * t42060 * t996 * t54983 + F::cast_from(0.39512695097613069591e1_f64) * t1076 * t3269 * t5015 * t3325 + F::cast_from(0.39512695097613069591e1_f64) * t16374 * t3067 - F::cast_from(0.19756347548806534796e1_f64) * t16305 * t3076 + F::cast_from(0.15805078039045227836e2_f64) * t1076 * t42067 * t1695 * t11122 - F::cast_from(0.19756347548806534796e1_f64) * t3047 * t16287 + F::cast_from(0.39512695097613069591e1_f64) * t3047 * t16328;
    t55405
}

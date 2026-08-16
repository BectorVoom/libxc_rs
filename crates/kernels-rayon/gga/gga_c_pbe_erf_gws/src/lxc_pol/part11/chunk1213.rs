//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1213/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1213(t12138: f64, t12234: f64, t13096: f64, t13127: f64, t13174: f64, t13217: f64, t13227: f64, t2408: f64, t2409: f64, t2503: f64, t35481: f64, t3913: f64, t3921: f64, t43887: f64, t43889: f64, t43903: f64, t46930: f64, t8589: f64, t9820: f64, t9890: f64, t9899: f64, t9902: f64) -> f64 {
    let t49219 = t2408 * t2409 * t8589 * t13227 / 4.0_f64 + t3921 * t12234 / 16.0_f64 + t13127 * t2503 / 24.0_f64 + t13174 * t2503 / 24.0_f64 + 35.0_f64 / 72.0_f64 * t35481 - 7.0_f64 / 72.0_f64 * t43887 - 7.0_f64 / 72.0_f64 * t43889 + 7.0_f64 / 3.0_f64 * t43903 - t9902 * t13217 / 24.0_f64 - t3913 * t9899 / 16.0_f64 + 3.0_f64 / 8.0_f64 * t3913 * t9820 + t46930 * t13096 / 16.0_f64 - t3913 * t12138 / 4.0_f64 - t3913 * t9890 / 8.0_f64;
    t49219
}

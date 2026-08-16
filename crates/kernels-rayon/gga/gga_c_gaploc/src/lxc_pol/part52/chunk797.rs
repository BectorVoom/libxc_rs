//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 797/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk797(t13194: f64, t1841: f64, t13200: f64, t13182: f64, t29439: f64, t33289: f64, t7810: f64, t9889: f64, t13055: f64, t28073: f64, t32840: f64, t3295: f64, t9805: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43095 = t1841 * t13194;
    let t43098 = t1841 * t13200;
    let t43100 = t29439 * t13182;
    let t43363 = t7810 * t33289 * t9889;
    let t43370 = t28073 * t13055;
    let t43373 = t9805 * t32840 * t3295;
    (t43095, t43098, t43100, t43363, t43370, t43373)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 810/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk810(t13176: f64, t731: f64, t13225: f64, t2549: f64, t2562: f64, t32179: f64, t883: f64, t943: f64, t33289: f64, t7810: f64, t9889: f64, t13055: f64, t28073: f64) -> (f64, f64, f64, f64, f64) {
    let t43290 = t731 * t13176;
    let t43326 = t2549 * t13225;
    let t43330 = t943 * t2562 * t883 * t32179;
    let t43363 = t7810 * t33289 * t9889;
    let t43370 = t28073 * t13055;
    (t43290, t43326, t43330, t43363, t43370)
}

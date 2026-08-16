//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 682/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk682(t13149: f64, t9438: f64, t825: f64, t10924: f64, t2558: f64, t9647: f64, t10628: f64, t5539: f64, t10697: f64, t3247: f64, t11167: f64, t2325: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13150 = t9438 * t13149;
    let t13151 = t825 * t13150;
    let t13182 = t10924 * t2558;
    let t13183 = t9647 * t13182;
    let t13194 = t5539 * t10628;
    let t13195 = t9647 * t13194;
    let t13200 = t10697 * t3247;
    let t13201 = t9647 * t13200;
    let t13258 = t2325 * t883 * t11167;
    (t13150, t13151, t13182, t13183, t13194, t13195, t13200, t13201, t13258)
}

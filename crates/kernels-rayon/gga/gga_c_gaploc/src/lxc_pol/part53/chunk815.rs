//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 815/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk815(t12696: f64, t5676: f64, t2033: f64, t2365: f64, t2610: f64, t9688: f64, t12695: f64, t549: f64, t12692: f64, t2013: f64, t10007: f64, t2530: f64, t825: f64, t9438: f64) -> (f64, f64, f64, f64, f64) {
    let t41286 = t5676 * t12696;
    let t41290 = t2033 * t2365 * t2610 * t9688;
    let t41293 = t2033 * t549 * t12695;
    let t41295 = t2013 * t12692;
    let t41299 = t825 * t9438 * t10007 * t2530;
    (t41286, t41290, t41293, t41295, t41299)
}

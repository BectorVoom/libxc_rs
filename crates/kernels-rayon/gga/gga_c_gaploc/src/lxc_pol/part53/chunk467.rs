//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 467/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk467(t7290: f64, t7291: f64, t123: f64, t2101: f64, t161: f64, t2610: f64, t1959: f64, t952: f64, t1: f64, t7275: f64, t787: f64, t588: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7292 = t7290 * t7291;
    let t7296 = t2101 * t123;
    let t7301 = t161 * t2610;
    let t7324 = t952 * t1959;
    let t7339 = t7275 * t1;
    let t7340 = t787 * t7339;
    let t7354 = t588 * t835;
    (t7292, t7296, t7301, t7324, t7340, t7354)
}

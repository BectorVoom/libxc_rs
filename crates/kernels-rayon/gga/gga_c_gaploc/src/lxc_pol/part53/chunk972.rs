//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 972/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk972(t47143: f64, t825: f64, t969: f64, t2365: f64, t39149: f64, t7390: f64, t47294: f64, t7584: f64, t7585: f64, t10930: f64, t10931: f64, t47243: f64) -> (f64, f64, f64, f64) {
    let t47344 = t825 * t969 * t47143;
    let t47347 = t7390 * t2365 * t39149;
    let t47357 = t7584 * t7585 * t47294;
    let t47360 = t10930 * t10931 * t47243;
    (t47344, t47347, t47357, t47360)
}

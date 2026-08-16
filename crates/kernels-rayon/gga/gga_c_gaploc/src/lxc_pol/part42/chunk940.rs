//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 940/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk940(t2617: f64, t3726: f64, t7803: f64, t2679: f64, t9796: f64, t12240: f64, t7810: f64, t47143: f64, t825: f64, t969: f64, t2365: f64, t39149: f64, t7390: f64) -> (f64, f64, f64, f64, f64) {
    let t47206 = t7803 * t3726 * t2617;
    let t47212 = t9796 * t3726 * t2679;
    let t47215 = t7810 * t12240 * t2617;
    let t47344 = t825 * t969 * t47143;
    let t47347 = t7390 * t2365 * t39149;
    (t47206, t47212, t47215, t47344, t47347)
}

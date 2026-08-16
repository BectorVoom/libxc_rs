//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1124/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1124(t1: f64, t106: f64, t13870: f64, t316: f64, t780: f64, t13858: f64, t2194: f64, t47143: f64, t825: f64, t969: f64, t2365: f64, t39149: f64, t7390: f64) -> (f64, f64, f64, f64) {
    let t47338 = t13870 * t1 * t106 * t316;
    let t47340 = 0.35750489951850426669e0_f64 * t780 * t47338;
    let t47341 = t2194 * t13858;
    let t47344 = t825 * t969 * t47143;
    let t47347 = t7390 * t2365 * t39149;
    (t47340, t47341, t47344, t47347)
}

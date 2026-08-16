//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 182/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk182(t1: f64, t231: f64, t362: f64, t46: f64, t375: f64, t268: f64, t378: f64, t61: f64) -> (f64, f64, f64, f64, f64) {
    let t643 = t231 * t1;
    let t645 = 0.18311555036753159941e-3_f64 * t643 * t362;
    let t646 = t231 * t46;
    let t648 = 0.58482233974552040708e0_f64 * t646 * t375;
    let t650 = t61 * t378 * t268;
    (t643, t645, t646, t648, t650)
}

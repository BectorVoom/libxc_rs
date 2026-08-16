//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 886/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk886(t1057: f64, t2742: f64, t1052: f64, t2747: f64, t496: f64, t5891: f64, t1056: f64, t457: f64, t1051: f64, t2750: f64, t1792: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7528 = t1057 * t2742;
    let t7530 = t1052 * t2747;
    let t7532 = t1057 * t2747;
    let t7535 = 24.0_f64 * t5891 * t496;
    let t7536 = t457 * t1056;
    let t7537 = t7536 * t496;
    let t7539 = t1051 * t2750;
    let t7540 = t7539 * t496;
    let t7543 = 1.0_f64 / t460 / t1792;
    (t7528, t7530, t7532, t7535, t7536, t7537, t7539, t7540, t7543)
}

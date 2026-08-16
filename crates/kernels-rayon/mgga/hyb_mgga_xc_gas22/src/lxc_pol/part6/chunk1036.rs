//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1036/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1036(t1117: f64, t2923: f64, t9604: f64, t9621: f64, t9624: f64, t9625: f64, t9629: f64, t9632: f64, t9636: f64, t9639: f64, t9642: f64, t9646: f64, t9650: f64, t9654: f64, t9657: f64, t9660: f64, t9663: f64, t9667: f64, t9670: f64) -> f64 {
    let t9677 = 1400.0_f64 / 3.0_f64 * t9621 * t9604 - 180.0_f64 * t9624 * t9625 * t2923 - 4.0_f64 * t1117 * t9629 + 800.0_f64 / 27.0_f64 * t9632 * t9636 + 800.0_f64 / 27.0_f64 * t9639 * t9636 - 128.0_f64 / 27.0_f64 * t9642 * t9646 - 64.0_f64 / 9.0_f64 * t9632 * t9650 + 128.0_f64 / 27.0_f64 * t9654 * t9657 + 64.0_f64 / 9.0_f64 * t9639 * t9660 - 64.0_f64 / 27.0_f64 * t9663 * t9650 + 128.0_f64 / 81.0_f64 * t9667 * t9657 + 64.0_f64 / 27.0_f64 * t9670 * t9660 + 800.0_f64 / 81.0_f64 * t9663 * t9636 + 800.0_f64 / 81.0_f64 * t9670 * t9636;
    t9677
}

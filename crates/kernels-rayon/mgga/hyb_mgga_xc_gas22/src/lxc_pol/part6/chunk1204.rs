//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1204/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1204(t2873: f64, t512: f64, t524: f64, t521: f64, t536: f64, t509: f64, t523: f64, t2880: f64, t2938: f64, t526: f64, t7572: f64, t527: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22639 = 1.0_f64 / t2873 / t512;
    let t22640 = t524 * t22639;
    let t22645 = t536 * t521;
    let t22652 = t523 * t509;
    let t22653 = t22652 * t521;
    let t22662 = t2938 * t2880;
    let t22703 = t7572 * t526;
    let t22705 = 1.0_f64 / t527 / t22703;
    (t22639, t22640, t22645, t22653, t22662, t22703, t22705)
}

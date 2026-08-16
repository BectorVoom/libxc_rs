//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 965/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk965(t848: f64, t8709: f64, t8651: f64, t6530: f64, t6533: f64, t6616: f64, t6619: f64, t6622: f64, t6698: f64, t8648: f64, t8654: f64, t8656: f64, t8659: f64, t8661: f64) -> (f64, f64, f64) {
    let t8869 = t8709 * t848;
    let t8877 = 0.103295e1_f64 * t8651;
    let t8887 = 0.1549425e1_f64 * t8648 - t8877 - 0.3529725e1_f64 * t8654 - 0.17648625e1_f64 * t8656 + 0.6311625e0_f64 * t8659 + 0.31558125e0_f64 * t8661 + 0.13772666666666666667e1_f64 * t6530 - 0.516475e0_f64 * t6533 - t6698 + 0.69463333333333333333e0_f64 * t6616 - 0.20839e0_f64 * t6619 - 0.20839e0_f64 * t6622;
    (t8869, t8877, t8887)
}

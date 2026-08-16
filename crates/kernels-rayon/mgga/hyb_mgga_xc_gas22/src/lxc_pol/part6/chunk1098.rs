//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1098/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1098(t10591: f64, t10593: f64, t10598: f64, t10602: f64, t10605: f64, t10609: f64, t10613: f64, t6616: f64, t6698: f64, t8706: f64, t8893: f64, t8894: f64) -> f64 {
    let t10741 = 0.31558125e0_f64 * t10591 + 0.6311625e0_f64 * t10593 - t6698 + 0.34731666666666666666e0_f64 * t6616 + 0.69463333333333333333e0_f64 * t8706 - t8893 - t8894 - 0.20839e0_f64 * t10598 + 0.62517e0_f64 * t10602 - 0.20839e0_f64 * t10605 + 0.312585e0_f64 * t10609 + 0.312585e0_f64 * t10613;
    t10741
}

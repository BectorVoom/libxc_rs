//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2551/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2551(t50846: f64, t51151: f64, t71146: f64, t71150: f64, t71152: f64, t71154: f64, t71156: f64, t71160: f64, t71166: f64, t71170: f64, t71174: f64, t71179: f64) -> f64 {
    let t71597 = -0.11072839506172839506e0_f64 * t71146 + 0.29896666666666666667e0_f64 * t71150 - 0.59793333333333333333e0_f64 * t71152 - 0.99655555555555555557e-1_f64 * t71154 + 0.39862222222222222223e0_f64 * t71156 - 0.73028148148148148149e0_f64 * t50846 + t51151 + 0.99655555555555555554e0_f64 * t71160 - 0.88582716049382716048e0_f64 * t71166 + 0.53814e1_f64 * t71170 + 0.71752e1_f64 * t71174 + 0.59793333333333333334e0_f64 * t71179;
    t71597
}

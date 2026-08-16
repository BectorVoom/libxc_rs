//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1513/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1513(t13788: f64, t2986: f64, t13528: f64, t4510: f64, t13532: f64, t10213: f64, t60: f64, t344: f64, t13537: f64, t10186: f64, t10192: f64, t10226: f64, t10229: f64, t13770: f64, t13782: f64, t13787: f64, t4511: f64, t4515: f64, t4519: f64) -> f64 {
    let t13790 = 0.18518518518518518518e-3_f64 * t2986 * t13788;
    let t13791 = t4510 * t13528;
    let t13794 = t4510 * t13532;
    let t13797 = t60 * t10213;
    let t13798 = t13797 * t344;
    let t13799 = t13798 * t13537;
    let t13804 = -0.18518518518518518518e-3_f64 * t10192 - 0.37037037037037037036e-3_f64 * t2986 * t13770 + 0.29629629629629629628e-2_f64 * t10186 * t4519 - 0.19753086419753086419e-2_f64 * t10186 * t4511 + 0.14814814814814814814e-2_f64 * t10186 * t4515 - t13782 + t13787 - t13790 + 0.74074074074074074072e-3_f64 * t2986 * t13791 + 0.37037037037037037036e-3_f64 * t2986 * t13794 + 0.86419753086419753084e-3_f64 * t2986 * t13799 - 0.12345679012345679012e-3_f64 * t10226 + 0.9259259259259259259e-4_f64 * t10229;
    t13804
}

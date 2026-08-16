//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1801/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1801(t10186: f64, t10192: f64, t10226: f64, t10229: f64, t13770: f64, t13782: f64, t13787: f64, t13790: f64, t13791: f64, t13794: f64, t13799: f64, t2986: f64, t4511: f64, t4515: f64, t4519: f64) -> f64 {
    let t13804 = -0.18518518518518518518e-3_f64 * t10192 - 0.37037037037037037036e-3_f64 * t2986 * t13770 + 0.29629629629629629628e-2_f64 * t10186 * t4519 - 0.19753086419753086419e-2_f64 * t10186 * t4511 + 0.14814814814814814814e-2_f64 * t10186 * t4515 - t13782 + t13787 - t13790 + 0.74074074074074074072e-3_f64 * t2986 * t13791 + 0.37037037037037037036e-3_f64 * t2986 * t13794 + 0.86419753086419753084e-3_f64 * t2986 * t13799 - 0.12345679012345679012e-3_f64 * t10226 + 0.9259259259259259259e-4_f64 * t10229;
    t13804
}

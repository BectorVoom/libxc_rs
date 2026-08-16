//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2157/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2157(t1336: f64, t22873: f64, t28171: f64, t28174: f64, t3777: f64, t5230: f64, t6420: f64, t7747: f64, t91002: f64, t91011: f64, t93605: f64, t93615: f64, t97119: f64, t97124: f64, t97129: f64, t97135: f64, t97137: f64, t97142: f64, t97146: f64, t97148: f64, t97152: f64) -> f64 {
    let t97154 = -t93605 + 0.3289868133696452873e-1_f64 * t97119 - t3777 * t28174 - t1336 * t22873 * t6420 - 0.76763589786250567037e-1_f64 * t97124 - 0.16449340668482264365e-1_f64 * t97129 + 2.0_f64 * t5230 * t7747 + 0.9869604401089358619e-1_f64 * t97135 + 0.38381794893125283518e-1_f64 * t97137 + 2.0_f64 * t3777 * t28171 + 0.41123351671205660912e-2_f64 * t97142 - t93615 - 0.23029076935875170111e0_f64 * t91002 - 0.6579736267392905746e-1_f64 * t97146 + 0.19190897446562641759e-1_f64 * t97148 + 0.16449340668482264365e-1_f64 * t97152 + t91011;
    t97154
}

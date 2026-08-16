//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1216/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1216(t82218: f64, t10115: f64, t24282: f64, t25168: f64, t2597: f64, t26728: f64, t82211: f64, t82221: f64, t82228: f64, t82230: f64, t82233: f64, t82236: f64, t82255: f64, t82259: f64) -> f64 {
    let t85129 = 0.55440370401180965083e0_f64 * t82218;
    let t85142 = -0.38381794893125283518e0_f64 * t82211 - t85129 + 0.9869604401089358619e-1_f64 * t82221 - 0.29608813203268075857e0_f64 * t82228 - 0.23029076935875170111e0_f64 * t82230 - 0.9869604401089358619e-1_f64 * t82233 - 0.24674011002723396548e-1_f64 * t82236 - 18.0_f64 * t25168 * t26728 * t10115 - 0.9869604401089358619e-1_f64 * t82255 + 0.38381794893125283518e0_f64 * t82259 - 3.0_f64 * t2597 * t24282;
    t85142
}

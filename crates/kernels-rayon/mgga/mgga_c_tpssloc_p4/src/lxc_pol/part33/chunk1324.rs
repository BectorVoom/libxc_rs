//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1324/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1324(t22986: f64, t23270: f64, t25191: f64, t5657: f64, t1528: f64, t17052: f64, t17090: f64, t21050: f64, t21053: f64, t25168: f64, t25169: f64, t259: f64, t28307: f64, t28432: f64, t4147: f64, t4268: f64, t5558: f64, t6627: f64, t7510: f64, t7517: f64, t7538: f64, t98239: f64, t98941: f64, t98966: f64, t98983: f64) -> (f64, f64) {
    let t105474 = t22986 * t23270 * t25191 * t5657;
    let t105508 = -0.23029076935875170111e0_f64 * t98941 - 18.0_f64 * t25168 * t25169 * t21053 + 3.0_f64 * t5558 * t7510 * t259 - 3.0_f64 * t4147 * t28432 - 0.24674011002723396548e-1_f64 * t98966 + 6.0_f64 * t17090 * t7517 - 3.0_f64 * t17052 * t7538 - 6.0_f64 * t98239 * t1528 - 6.0_f64 * t6627 * t21050 + 0.12337005501361698274e-1_f64 * t98983 + 12.0_f64 * t4268 * t28307;
    (t105474, t105508)
}

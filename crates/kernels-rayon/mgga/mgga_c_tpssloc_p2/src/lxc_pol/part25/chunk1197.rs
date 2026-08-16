//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1197/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1197(t81146: f64, t81153: f64, t12168: f64, t1336: f64, t7208: f64, t81115: f64, t81122: f64, t81125: f64, t81127: f64, t81132: f64, t81140: f64, t81149: f64, t81157: f64, t81160: f64, t81165: f64, t81169: f64, t81173: f64, t81177: f64, t81181: f64, t81184: f64) -> f64 {
    let t84595 = 0.27415567780803773942e-2_f64 * t81146;
    let t84597 = 0.19739208802178717238e0_f64 * t81153;
    let t84606 = 0.24674011002723396548e-1_f64 * t81115 - 0.49348022005446793095e-1_f64 * t81122 + 0.24674011002723396548e-1_f64 * t81125 + 0.23029076935875170111e0_f64 * t81127 - 0.9869604401089358619e-1_f64 * t81132 - t1336 * t7208 * t12168 - 0.14804406601634037928e0_f64 * t81140 - t84595 - 0.49348022005446793095e-1_f64 * t81149 + t84597 + 0.16449340668482264365e-1_f64 * t81157 - 0.46058153871750340221e0_f64 * t81160 - 0.29608813203268075857e0_f64 * t81165 + 0.9869604401089358619e-1_f64 * t81169 + 0.9869604401089358619e-1_f64 * t81173 - 0.16449340668482264365e-1_f64 * t81177 + 0.9869604401089358619e-1_f64 * t81181 - 0.23029076935875170111e0_f64 * t81184;
    t84606
}

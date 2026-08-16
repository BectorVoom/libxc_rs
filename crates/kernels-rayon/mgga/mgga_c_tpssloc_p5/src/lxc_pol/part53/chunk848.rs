//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 848/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk848(t26429: f64, t1338: f64, t7918: f64, t1352: f64, t5287: f64, t7208: f64, t27051: f64, t553: f64, t1332: f64, t1336: f64, t1814: f64, t2089: f64, t22728: f64, t22731: f64, t22746: f64, t22753: f64, t22896: f64, t24108: f64, t24110: f64, t26434: f64, t26437: f64, t26449: f64, t26463: f64, t26468: f64, t5230: f64, t544: f64, t7211: f64, t7934: f64) -> f64 {
    let t27096 = 0.38381794893125283518e-1_f64 * t26429;
    let t27097 = t1338 * t7918;
    let t27098 = t27097 * t1352;
    let t27103 = t7208 * t5287;
    let t27105 = t553 * t27051;
    let t27113 = -t27096 - t1336 * t27098 + 0.16449340668482264365e-1_f64 * t26434 - 0.82246703342411321825e-2_f64 * t26437 + t24108 + t24110 - 0.82246703342411321825e-2_f64 * t22728 - t22731 - t1336 * t27103 + t544 * t27105 + 0.9869604401089358619e-1_f64 * t26449 + t22746 + t22753 + t1332 * t7934 - 0.16449340668482264365e-1_f64 * t26463 + t1814 * t7211 + t22896 + t5230 * t2089 - 0.16449340668482264365e-1_f64 * t26468;
    t27113
}

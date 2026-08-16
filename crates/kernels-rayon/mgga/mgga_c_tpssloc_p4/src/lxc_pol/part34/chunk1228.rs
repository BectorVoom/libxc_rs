//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1228/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1228(t108249: f64, t108268: f64, t108290: f64, t108309: f64, t101708: f64, t105661: f64, t105665: f64, t105669: f64, t105674: f64, t105685: f64, t13228: f64, t1499: f64, t20853: f64, t226: f64, t235: f64, t29041: f64, t4281: f64, t5575: f64, t7101: f64, t7839: f64, t812: f64, t84995: f64, t85003: f64, t85027: f64, t87635: f64, t87653: f64, t87666: f64, t87718: f64, t98564: f64, t98884: f64) -> (f64, f64) {
    let t108311 = t108249 + t108268 + t108290 + t108309;
    let t108321 = 0.23029076935875170111e0_f64 * t98564 + 6.0_f64 * t4281 * t101708 * t13228 - t84995 - 0.76763589786250567036e0_f64 * t87635 - 0.49348022005446793095e-1_f64 * t87653 + t85003 + 3.0_f64 * t5575 * t7839 + 0.9869604401089358619e-1_f64 * t105661 + 0.19739208802178717238e0_f64 * t105665 + 0.9869604401089358619e-1_f64 * t105669 + 3.0_f64 * t1499 * t29041 + t226 * t235 * t108311 - t812 * t7101 * t20853 - 0.38381794893125283518e0_f64 * t87666 - 0.39478417604357434476e0_f64 * t105674 + 0.9869604401089358619e-1_f64 * t105685 - t85027 - 0.31253747270116302294e0_f64 * t87718 + 0.24674011002723396548e-1_f64 * t98884;
    (t108311, t108321)
}

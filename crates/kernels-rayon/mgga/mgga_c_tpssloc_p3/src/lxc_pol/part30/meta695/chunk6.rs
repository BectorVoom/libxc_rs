//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2225/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2225(t17046: f64, t1888: f64, t6646: f64, t1510: f64, t22986: f64, t87130: f64, t25249: f64, t4234: f64, t23110: f64, t28337: f64, t81651: f64, t13176: f64, t1499: f64, t22992: f64, t25295: f64, t5617: f64, t7533: f64, t812: f64, t81595: f64, t81599: f64, t81600: f64, t81602: f64, t92513: f64, t98416: f64, t98420: f64, t98425: f64, t98428: f64, t98432: f64) -> f64 {
    let t98435 = t1888 * t6646 * t17046;
    let t98439 = t22986 * t6646 * t87130 * t1510;
    let t98443 = t22986 * t6646 * t25249 * t4234;
    let t98446 = t81651 * t23110 * t28337;
    let t98450 = -0.82246703342411321824e-2_f64 * t81595 + 2.0_f64 * t1499 * t25295 - t81599 + 0.26044789391763585244e-1_f64 * t81600 + 0.63969658155208805863e-1_f64 * t81602 + 0.76763589786250567037e-1_f64 * t98416 + t92513 - 2.0_f64 * t13176 * t7533 - 0.76763589786250567037e-1_f64 * t98420 + 0.16449340668482264365e-1_f64 * t98425 - 0.16449340668482264365e-1_f64 * t98428 + 0.16449340668482264365e-1_f64 * t98432 - 0.82246703342411321825e-2_f64 * t98435 + 0.3289868133696452873e-1_f64 * t98439 + 0.3289868133696452873e-1_f64 * t98443 - 0.16449340668482264365e-1_f64 * t98446 - t812 * t22992 * t5617;
    t98450
}

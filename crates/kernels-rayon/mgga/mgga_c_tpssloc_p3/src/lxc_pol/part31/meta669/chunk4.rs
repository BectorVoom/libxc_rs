//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1981/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1981(t16753: f64, t26662: f64, t4166: f64, t5575: f64, t7101: f64, t7104: f64, t812: f64, t87167: f64, t87177: f64, t92551: f64, t92556: f64, t92560: f64, t92561: f64, t92564: f64, t92565: f64, t98505: f64, t98513: f64, t98516: f64, t98520: f64, t98530: f64, t98534: f64) -> f64 {
    let t101687 = t87167 + 0.76763589786250567037e-1_f64 * t98505 + 0.9869604401089358619e-1_f64 * t98513 - 0.49348022005446793095e-1_f64 * t98516 - 0.6579736267392905746e-1_f64 * t98520 + 0.3289868133696452873e-1_f64 * t87177 - t92551 + t92556 - 0.16449340668482264365e-1_f64 * t98530 + t92560 + t5575 * t7104 + 0.3289868133696452873e-1_f64 * t98534 + t92561 - t92564 - t92565 - t812 * t7101 * t16753 - 2.0_f64 * t4166 * t26662;
    t101687
}

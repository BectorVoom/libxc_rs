//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1984/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1984(t101715: f64, t13397: f64, t16816: f64, t16830: f64, t17034: f64, t26657: f64, t26676: f64, t4182: f64, t4281: f64, t82032: f64, t85027: f64, t87687: f64, t87708: f64, t87718: f64, t92798: f64, t92810: f64, t92811: f64, t92822: f64, t92825: f64, t98601: f64, t98608: f64, t98881: f64, t98884: f64) -> f64 {
    let t101751 = 6.0_f64 * t4281 * t101715 * t4182 + t92798 - t87687 - 2.0_f64 * t16830 * t26676 - 0.52089578783527170489e-1_f64 * t82032 - 0.3289868133696452873e-1_f64 * t98601 - t87708 - 6.0_f64 * t13397 * t101715 * t16816 + t92810 + 4.0_f64 * t17034 * t26657 - t92811 - t85027 + 0.6579736267392905746e-1_f64 * t98608 - 0.20835831513410868196e0_f64 * t87718 + t92822 + 0.9869604401089358619e-1_f64 * t98881 + 0.82246703342411321825e-2_f64 * t98884 - t92825;
    t101751
}

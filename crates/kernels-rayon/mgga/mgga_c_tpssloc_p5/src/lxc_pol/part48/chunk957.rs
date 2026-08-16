//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 957/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk957(t23012: f64, t8538: f64, t31339: f64, t81591: f64, t10049: f64, t112687: f64, t112697: f64, t112700: f64, t112703: f64, t114632: f64, t114648: f64, t114668: f64, t114695: f64, t114754: f64, t1911: f64, t2054: f64, t22975: f64, t22979: f64, t24281: f64, t24282: f64, t24305: f64, t24314: f64, t2713: f64, t2718: f64, t31343: f64, t6627: f64, t6663: f64, t7087: f64, t82287: f64, t855: f64, t8553: f64, t858: f64) -> f64 {
    let t114759 = t23012 * t8538;
    let t114760 = 0.63969658155208805863e-1_f64 * t114759;
    let t114762 = t81591 * t31339;
    let t114764 = -t112687 + 4.0_f64 * t2713 * t31343 + 2.0_f64 * t10049 * t8553 + 2.0_f64 * t7087 * t22975 - 6.0_f64 * t6627 * t24314 + 2.0_f64 * t855 * t2718 * t24281 * t1911 + 0.16449340668482264365e-1_f64 * t114632 - t112697 + t112700 - t112703 - 2.0_f64 * t24305 * t6663 + 4.0_f64 * t7087 * t22979 - 2.0_f64 * t82287 * t2054 - t855 * t858 * (t114648 + t114668 + t114695 + t114754) + t114760 - t6627 * t24282 - 0.76763589786250567036e-1_f64 * t114762;
    t114764
}

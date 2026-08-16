//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2018/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2018(t5230: f64, t7934: f64, t90980: f64, t93588: f64, t93589: f64, t93590: f64, t93592: f64, t93599: f64, t93600: f64, t97079: f64, t97083: f64, t97087: f64, t97091: f64, t97095: f64, t97106: f64, t97108: f64, t97111: f64, t97114: f64) -> f64 {
    let t102629 = 2.0_f64 * t5230 * t7934 + t93588 - t93589 - t93590 - 0.16449340668482264365e-1_f64 * t97079 + 0.6579736267392905746e-1_f64 * t97083 + 0.6579736267392905746e-1_f64 * t97087 + 0.6579736267392905746e-1_f64 * t97091 + t93592 + 0.15352717957250113407e0_f64 * t97095 + 0.3289868133696452873e-1_f64 * t90980 + t93599 - t93600 + 0.6579736267392905746e-1_f64 * t97106 + 0.76763589786250567037e-1_f64 * t97108 - 0.82246703342411321825e-2_f64 * t97111 - 0.16449340668482264365e-1_f64 * t97114;
    t102629
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1848/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1848(t87583: f64, t87601: f64, t87603: f64, t87612: f64, t87618: f64, t87668: f64, t87679: f64, t87709: f64, t87714: f64, t87729: f64, t87733: f64, t87753: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92739 = 0.15352717957250113407e0_f64 * t87583;
    let t92749 = 0.16449340668482264365e-1_f64 * t87601;
    let t92754 = 0.15352717957250113407e0_f64 * t87603;
    let t92760 = 0.3289868133696452873e-1_f64 * t87612;
    let t92768 = 0.3289868133696452873e-1_f64 * t87618;
    let t92795 = 0.76763589786250567036e-1_f64 * t87668;
    let t92798 = 0.3289868133696452873e-1_f64 * t87679;
    let t92810 = 0.76763589786250567036e-1_f64 * t87709;
    let t92811 = 0.9869604401089358619e-1_f64 * t87714;
    let t92822 = 0.16449340668482264365e-1_f64 * t87729;
    let t92825 = 0.76763589786250567036e-1_f64 * t87733;
    let t92846 = 0.3289868133696452873e-1_f64 * t87753;
    (t92739, t92749, t92754, t92760, t92768, t92795, t92798, t92810, t92811, t92822, t92825, t92846)
}

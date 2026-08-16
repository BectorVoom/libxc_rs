//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2212/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2212(t25038: f64, t25040: f64, t86873: f64, t17052: f64, t6663: f64, t82070: f64, t82082: f64, t86929: f64, t92406: f64, t98189: f64, t98192: f64, t98196: f64, t98199: f64, t98202: f64) -> f64 {
    let t98205 = t25038 * t86873 * t25040;
    let t98208 = 0.16449340668482264365e-1_f64 * t98189 + t92406 + 0.3289868133696452873e-1_f64 * t98192 - t17052 * t6663 + t82070 + 0.3289868133696452873e-1_f64 * t98196 - 0.16449340668482264365e-1_f64 * t98199 - 0.16449340668482264365e-1_f64 * t98202 + 0.9869604401089358619e-1_f64 * t98205 - t86929 + 0.82246703342411321824e-2_f64 * t82082;
    t98208
}

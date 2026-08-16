//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 963/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk963(t118766: f64, t30676: f64, t5544: f64, t6552: f64, t6637: f64, t23035: f64, t5527: f64, t118915: f64, t118927: f64, t118934: f64, t118940: f64, t1408: f64, t7540: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t126484 = 0.16449340668482264365e-1_f64 * t118766;
    let t126488 = 0.3289868133696452873e-1_f64 * t6552 * t6637 * t30676 * t5544;
    let t126492 = 0.9869604401089358619e-1_f64 * t23035 * t6637 * t30676 * t5527;
    let t126497 = 0.76763589786250567036e-1_f64 * t118915;
    let t126518 = 0.76763589786250567036e-1_f64 * t118927;
    let t126520 = 0.16449340668482264365e-1_f64 * t118934;
    let t126521 = 0.3289868133696452873e-1_f64 * t118940;
    let t126530 = t1408 * t7540;
    (t126484, t126488, t126492, t126497, t126518, t126520, t126521, t126530)
}

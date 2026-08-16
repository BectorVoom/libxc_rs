//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2228/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2228(t25416: f64, t82431: f64, t1921: f64, t88804: f64, t23384: f64, t25811: f64, t1052: f64, t14526: f64, t1920: f64, t1927: f64, t225: f64, t23327: f64, t23329: f64, t23336: f64, t23725: f64, t25453: f64, t25738: f64, t25749: f64, t25816: f64, t2776: f64, t3026: f64, t3174: f64, t345: f64, t387: f64, t388: f64, t4552: f64, t4660: f64, t4693: f64, t6687: f64, t6768: f64, t6815: f64, t7553: f64, t82357: f64, t82402: f64, t83342: f64, t83344: f64, t986: f64) -> f64 {
    let t88915 = 0.18277045187202515961e-2_f64 * t82431 * t25416;
    let t88932 = t1921 * t88804;
    let t88937 = 0.18277045187202515961e-2_f64 * t23384 * t25811;
    let t88940 = 0.82246703342411321825e-2_f64 * t1920 * t345 * t14526 * t225 * t387 + 0.26806332941230356743e-1_f64 * t83342 + 0.97477574331746751793e-2_f64 * t83344 + 0.14621636149762012769e-1_f64 * t82402 * t25816 - 0.3289868133696452873e-1_f64 * t1927 * t23336 * t25738 - t88915 + 0.54831135561607547884e-2_f64 * t23327 * t23329 * t25749 * t2776 - 0.27415567780803773942e-2_f64 * t23327 * t82357 * t7553 + 2.0_f64 * t4552 * t6768 * t388 + 4.0_f64 * t1052 * t3174 * t6815 * t4693 + 4.0_f64 * t4660 * t23725 + 0.16449340668482264365e-1_f64 * t6687 * t986 * t88932 + t88937 + 4.0_f64 * t3026 * t25453;
    t88940
}

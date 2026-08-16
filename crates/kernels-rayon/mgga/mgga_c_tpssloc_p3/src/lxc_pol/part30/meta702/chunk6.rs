//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2279/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2279(t23384: f64, t28519: f64, t1052: f64, t23329: f64, t23346: f64, t25429: f64, t25453: f64, t25778: f64, t28510: f64, t28593: f64, t28679: f64, t3026: f64, t3174: f64, t388: f64, t4660: f64, t4665: f64, t4693: f64, t5943: f64, t6815: f64, t7624: f64, t82411: f64, t83344: f64, t88889: f64, t88915: f64, t990: f64, t99099: f64) -> f64 {
    let t99439 = t23384 * t28519;
    let t99450 = -t88889 + 4.0_f64 * t25778 * t4665 + t990 * t28593 * t388 + 4.0_f64 * t1052 * t3174 * t7624 * t4693 + 0.48738787165873375896e-2_f64 * t83344 - 0.36554090374405031923e-2_f64 * t25429 * t23329 * t82411 * t99099 - t88915 + 0.21932454224643019153e-1_f64 * t23346 * t28519 - 0.27415567780803773942e-2_f64 * t99439 - 0.14621636149762012769e-1_f64 * t23346 * t28510 - t3026 * t28679 + 2.0_f64 * t1052 * t3174 * t6815 * t5943 + 4.0_f64 * t4660 * t25453;
    t99450
}

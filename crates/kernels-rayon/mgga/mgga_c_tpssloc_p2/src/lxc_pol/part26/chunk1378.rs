//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1378/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1378(t11553: f64, t2121: f64, t2123: f64, t2122: f64, t85628: f64, t24574: f64, t24884: f64, t11154: f64, t11613: f64, t1186: f64, t11928: f64, t2155: f64, t24582: f64, t24595: f64, t24893: f64, t3471: f64, t3487: f64, t3631: f64, t45345: f64, t45355: f64, t466: f64, t498: f64, t7283: f64, t7286: f64, t7295: f64, t7356: f64, t86376: f64) -> f64 {
    let t86451 = 0.30461741978670859935e-2_f64 * t2121 * t11553 * t2123;
    let t86452 = t2122 * t85628;
    let t86456 = t24574 * t24884;
    let t86468 = -0.24674011002723396548e-1_f64 * t7283 * t3471 * t7295 + 6.0_f64 * t11928 * t7356 + 12.0_f64 * t3487 * t24582 - 3.0_f64 * t45345 * t2155 + 12.0_f64 * t11613 * t7356 + t86451 + 0.24674011002723396548e-1_f64 * t7283 * t1186 * t86452 - 0.27415567780803773942e-2_f64 * t86456 + t466 * t86376 * t498 - 3.0_f64 * t24893 * t3631 - 3.0_f64 * t45355 * t2155 + 0.21932454224643019154e-1_f64 * t7283 * t24595 * t7286 * t11154;
    t86468
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1370/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1370(t11745: f64, t24729: f64, t2132: f64, t24746: f64, t86192: f64, t11638: f64, t11655: f64, t11766: f64, t11770: f64, t2133: f64, t2136: f64, t24650: f64, t24655: f64, t24733: f64, t3469: f64, t475: f64, t68: f64, t7310: f64, t7321: f64, t7326: f64, t7328: f64, t7331: f64, t7345: f64, t83100: f64, t86293: f64, t86296: f64) -> f64 {
    let t86299 = t24729 * t11745;
    let t86313 = t2132 * t86192 * t24746;
    let t86317 = -t24733 * t11770 / 512.0_f64 + 0.10093189023535097714e-3_f64 * t7326 * t7328 * t11638 * t68 * t475 - 0.60559134141210586284e-3_f64 * t86293 - 0.30279567070605293142e-3_f64 * t86296 * t7331 + t86299 / 384.0_f64 + 5.0_f64 / 1152.0_f64 * t7345 * t11655 - 0.30279567070605293142e-3_f64 * t2132 * t2133 * t3469 * t7321 - 0.30279567070605293142e-3_f64 * t24650 * t24655 - 0.10093189023535097714e-3_f64 * t2132 * t83100 * t2136 + 0.60559134141210586284e-3_f64 * t86313 - 7.0_f64 / 648.0_f64 * t7310 * t11766;
    t86317
}

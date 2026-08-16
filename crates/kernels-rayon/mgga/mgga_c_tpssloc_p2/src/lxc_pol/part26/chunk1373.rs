//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1373/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1373(t11761: f64, t11850: f64, t11858: f64, t2140: f64, t24699: f64, t24749: f64, t488: f64, t7310: f64, t7316: f64, t7321: f64, t86348: f64, t86350: f64, t86354: f64, t86357: f64, t86365: f64, t86368: f64) -> f64 {
    let t86373 = t86348 / 3456.0_f64 - t86350 / 2304.0_f64 + t7310 * t11761 / 36.0_f64 - t86354 / 576.0_f64 - 0.30279567070605293142e-3_f64 * t86357 - t7310 * t11850 / 48.0_f64 + 0.30279567070605293142e-3_f64 * t24749 * t7321 + 0.30279567070605293142e-3_f64 * t7316 * t24699 + t86365 / 216.0_f64 - 0.30279567070605293142e-3_f64 * t86368 + t11858 * t2140 * t488 / 1536.0_f64;
    t86373
}

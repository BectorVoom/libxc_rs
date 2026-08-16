//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1273/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1273(t1528: f64, t1912: f64, t259: f64, t4147: f64, t4268: f64, t6549: f64, t6565: f64, t6627: f64, t7481: f64, t7486: f64, t7490: f64, t7492: f64, t7511: f64, t7517: f64, t7538: f64, t855: f64) -> f64 {
    let t7540 = -t6549 - 0.16449340668482264365e-1_f64 * t7481 - t6565 + 0.82246703342411321825e-2_f64 * t7486 - 0.82246703342411321825e-2_f64 * t7490 + t7492 * t259 + t7511 * t259 - t6627 * t1528 - t4147 * t1912 - t4268 * t1912 + 2.0_f64 * t855 * t7517 - t855 * t7538;
    t7540
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 657/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk657(t8562: f64, t858: f64, t1912: f64, t2054: f64, t259: f64, t6627: f64, t7087: f64, t8334: f64, t8338: f64, t8539: f64, t8544: f64, t8549: f64, t855: f64, t8553: f64) -> (f64, f64) {
    let t8563 = t858 * t8562;
    let t8565 = t8334 - t8338 + 0.82246703342411321825e-2_f64 * t8539 + t8544 * t259 - t7087 * t1912 - 0.82246703342411321825e-2_f64 * t8549 - t6627 * t2054 + 2.0_f64 * t855 * t8553 - t855 * t8563;
    (t8563, t8565)
}

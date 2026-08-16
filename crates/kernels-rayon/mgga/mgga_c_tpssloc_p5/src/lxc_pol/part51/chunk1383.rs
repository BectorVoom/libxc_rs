//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1383/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1383(t114760: f64, t114762: f64, t118526: f64, t118626: f64, t118630: f64, t118633: f64, t121367: f64, t121371: f64, t121382: f64, t121391: f64, t1527: f64, t23281: f64, t25168: f64, t25199: f64, t26728: f64, t2718: f64, t31399: f64, t7516: f64, t7830: f64, t855: f64, t865: f64, t92394: f64) -> f64 {
    let t121393 = 0.16449340668482264365e-1_f64 * t121367 + 2.0_f64 * t23281 * t7830 - 0.38381794893125283518e-1_f64 * t121371 - t118526 + 24.0_f64 * t25168 * t92394 * t7516 * t865 - t118626 - 6.0_f64 * t25168 * t26728 * t25199 + t114760 + t118630 - t118633 - 0.38381794893125283518e-1_f64 * t114762 + 0.16449340668482264365e-1_f64 * t121382 + 2.0_f64 * t855 * t2718 * t31399 * t1527 + 0.82246703342411321825e-2_f64 * t121391;
    t121393
}

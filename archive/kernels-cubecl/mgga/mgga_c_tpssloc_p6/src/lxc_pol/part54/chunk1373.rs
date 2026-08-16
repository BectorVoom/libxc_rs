//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1373/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1373<F: Float>(t114760: F, t114762: F, t118526: F, t118626: F, t118630: F, t118633: F, t121367: F, t121371: F, t121382: F, t121391: F, t1527: F, t23281: F, t25168: F, t25199: F, t26728: F, t2718: F, t31399: F, t7516: F, t7830: F, t855: F, t865: F, t92394: F) -> F {
    let t121393 = F::cast_from(0.16449340668482264365e-1_f64) * t121367 + F::cast_from(2.0_f64) * t23281 * t7830 - F::cast_from(0.38381794893125283518e-1_f64) * t121371 - t118526 + F::cast_from(24.0_f64) * t25168 * t92394 * t7516 * t865 - t118626 - F::cast_from(6.0_f64) * t25168 * t26728 * t25199 + t114760 + t118630 - t118633 - F::cast_from(0.38381794893125283518e-1_f64) * t114762 + F::cast_from(0.16449340668482264365e-1_f64) * t121382 + F::cast_from(2.0_f64) * t855 * t2718 * t31399 * t1527 + F::cast_from(0.82246703342411321825e-2_f64) * t121391;
    t121393
}

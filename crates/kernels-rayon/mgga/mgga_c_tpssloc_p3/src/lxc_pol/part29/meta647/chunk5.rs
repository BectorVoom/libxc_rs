//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2147/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2147(t13336: f64, t1909: f64, t25269: f64, t2617: f64, t4182: f64, t4281: f64, t7533: f64, t81980: f64, t81989: f64, t82005: f64, t82013: f64, t82016: f64, t87620: f64, t87660: f64, t87666: f64, t87669: f64, t87672: f64, t87676: f64, t87680: f64, t87687: f64, t87692: f64, t9612: f64) -> f64 {
    let t87694 = 0.16449340668482264365e-1_f64 * t87660 - 0.11514538467937585055e0_f64 * t81980 + 0.38381794893125283518e-1_f64 * t81989 + 0.38381794893125283518e-1_f64 * t82005 + t13336 * t1909 - 0.63969658155208805863e-1_f64 * t87666 + t87669 - 0.3289868133696452873e-1_f64 * t87672 - 0.16449340668482264365e-1_f64 * t87676 + t87680 + 4.0_f64 * t4281 * t87620 * t4182 - t9612 * t7533 - 2.0_f64 * t2617 * t25269 - t87687 - 0.38381794893125283518e-1_f64 * t82013 - 0.82246703342411321824e-2_f64 * t82016 - 0.82246703342411321825e-2_f64 * t87692;
    t87694
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 995/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk995(t112676: f64, t121296: f64, t121305: f64, t121308: f64, t121454: f64, t126226: f64, t126229: f64, t126233: f64, t126240: f64, t126246: f64, t127778: f64, t127786: f64, t127790: f64, t127794: f64, t1528: f64, t17052: f64, t17092: f64, t33399: f64, t4147: f64, t8563: f64) -> f64 {
    let t127796 = -t126226 + t126229 + 0.38381794893125283518e-1_f64 * t121296 + 0.82246703342411321824e-2_f64 * t121305 + t126233 - 0.16449340668482264365e-1_f64 * t121308 - 2.0_f64 * t4147 * t33399 - 0.82246703342411321825e-2_f64 * t127778 + t126240 - 2.0_f64 * t17092 * t8563 - 2.0_f64 * t121454 * t1528 - t17052 * t8563 - 0.16449340668482264365e-1_f64 * t127786 - 0.49348022005446793095e-1_f64 * t127790 + t126246 + 0.3289868133696452873e-1_f64 * t127794 - t112676;
    t127796
}

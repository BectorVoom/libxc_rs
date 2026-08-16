//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 853/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk853(t31104: f64, t6897: f64, t6883: f64, t8455: f64, t8459: f64, t22674: f64, t8458: f64, t2006: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31106 = 0.82246703342411321825e-2_f64 * t6897 * t31104;
    let t31113 = 0.38381794893125283518e-1_f64 * t6883 * t8455;
    let t31115 = 0.38381794893125283518e-1_f64 * t6883 * t8459;
    let t31127 = t22674 * t8458;
    let t31129 = 0.82246703342411321825e-2_f64 * t6897 * t31127;
    let t31137 = t214 * t2006;
    (t31106, t31113, t31115, t31127, t31129, t31137)
}

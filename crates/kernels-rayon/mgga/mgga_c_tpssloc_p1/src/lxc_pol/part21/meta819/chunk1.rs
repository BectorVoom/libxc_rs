//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2884/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2884(t13637: f64, t60237: f64, t41959: f64, t41962: f64, t59680: f64, t59684: f64, t59688: f64, t59692: f64, t59694: f64, t60223: f64, t60226: f64, t60229: f64, t60232: f64, t60235: f64, t60238: f64) -> (f64, f64) {
    let t60240 = t13637 * t60237;
    let t60242 = 0.20128333333333333334e0_f64 * t59680 - 0.301925e0_f64 * t59684 + 0.26837777777777777777e0_f64 * t59688 + 0.12077e1_f64 * t59692 - 0.13418888888888888889e0_f64 * t59694 - 0.5519e-1_f64 * t60223 - 0.27595e-1_f64 * t60226 - 0.36793333333333333333e-1_f64 * t60229 - 0.99342e0_f64 * t60232 - 0.49671e0_f64 * t60235 + t41959 + t41962 + 0.776775e1_f64 * t60238 - 0.16504875e0_f64 * t60240;
    (t60240, t60242)
}

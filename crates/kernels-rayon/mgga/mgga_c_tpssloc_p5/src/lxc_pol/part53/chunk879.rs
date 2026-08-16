//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 879/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk879(t31993: f64, t829: f64, t235: f64, t31984: f64, t226: f64, t31379: f64, t31387: f64, t31391: f64, t31987: f64, t31989: f64, t808: f64, t812: f64, t8738: f64) -> (f64, f64, f64) {
    let t31994 = t31993 * t829;
    let t31996 = t235 * t31984;
    let t31998 = -t31987 - 0.6579736267392905746e-1_f64 * t31379 - t31989 - 0.3289868133696452873e-1_f64 * t31387 + 0.3289868133696452873e-1_f64 * t31391 + t808 * t8738 - t812 * t31994 + t226 * t31996;
    (t31994, t31996, t31998)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 859/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk859(t69279: f64, t75346: f64, t75351: f64, t25525: f64, t3068: f64, t75355: f64, t2044: f64, t25640: f64, t75359: f64, t69276: f64, t75318: f64, t75321: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75378 = t69279 * t75346;
    let t75380 = t69279 * t75351;
    let t75383 = t25525 * t3068 * t75355;
    let t75386 = t25640 * t2044 * t75359;
    let t75388 = t69276 * t75318;
    let t75390 = t69276 * t75321;
    (t75378, t75380, t75383, t75386, t75388, t75390)
}

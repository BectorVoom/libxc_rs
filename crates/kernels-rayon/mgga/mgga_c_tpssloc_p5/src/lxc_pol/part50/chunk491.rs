//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 491/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk491(t2928: f64, t315: f64, t323: f64, t300: f64, t938: f64, t964: f64, t969: f64, t615: f64, t972: f64, t340: f64, t697: f64, t344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2929 = 1.0_f64 / t2928;
    let t2930 = t315 * t2929;
    let t2931 = t323 * t323;
    let t2932 = 1.0_f64 / t2931;
    let t2940 = t300 * t938;
    let t2958 = t964 * t969;
    let t2960 = t615 * t972;
    let t2965 = t697 * t340;
    let t2966 = t2965 * t344;
    (t2929, t2930, t2932, t2940, t2958, t2960, t2965, t2966)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2920/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2920(t17299: f64, t2940: f64, t13659: f64, t4483: f64, t17947: f64, t2907: f64, t959: f64, t17191: f64, t300: f64, t961: f64, t13724: f64, t17564: f64, t42671: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60842 = 0.11696447245269292414e1_f64 * t2940 * t17299;
    let t60844 = 0.69263436422725855034e2_f64 * t4483 * t13659;
    let t60847 = 0.14035736694323150897e2_f64 * t959 * t17947 * t2907;
    let t60848 = t300 * t17191;
    let t60850 = 0.11696447245269292414e1_f64 * t60848 * t961;
    let t60852 = 0.2077903092681775651e3_f64 * t4483 * t13724;
    let t60855 = 0.10254018858216406658e4_f64 * t959 * t17564 * t42671;
    (t60842, t60844, t60847, t60850, t60852, t60855)
}

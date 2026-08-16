//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2079/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2079(t26211: f64, t6883: f64, t268: f64, t557: f64, t6559: f64, t26333: f64, t81326: f64, t22642: f64, t22643: f64, t7700: f64, t22674: f64, t26202: f64, t6897: f64) -> (f64, f64, f64, f64, f64) {
    let t90604 = t6883 * t26211;
    let t90605 = 0.38381794893125283518e-1_f64 * t90604;
    let t90607 = t6559 * t557 * t268;
    let t90609 = t90607 * t81326 * t26333;
    let t90642 = t22642 * t22643 * t7700;
    let t90645 = t6897 * t22674 * t26202;
    (t90605, t90607, t90609, t90642, t90645)
}

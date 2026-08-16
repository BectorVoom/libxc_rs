//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1196/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1196(t2678: f64, t852: f64, t225: f64, t9520: f64, t3639: f64, t11923: f64, t11931: f64, t11604: f64, t496: f64, t68: f64, t11601: f64, t11599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40955 = t852 * t2678;
    let t41554 = t9520 * t225;
    let t43705 = t3639 * t3639;
    let t43706 = 1.0_f64 / t43705;
    let t44412 = t11923 * t225;
    let t45345 = t11931 * t225;
    let t45349 = 1.0_f64 / t11604 / t496;
    let t45350 = t68 * t45349;
    let t45355 = t11601 * t225;
    let t45375 = t11599 * t225;
    (t40955, t41554, t43706, t44412, t45345, t45350, t45355, t45375)
}

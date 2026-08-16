//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 602/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk602(t1915: f64, t25: f64, t1877: f64, t337: f64, t38: f64, t1887: f64) -> (f64, f64, f64) {
    let t1916 = t1915 * t25;
    let t1918 = t1877 * t1916 / 2.0_f64;
    let t1919 = t38 * t337;
    let t1920 = t1919 * t1887;
    (t1918, t1919, t1920)
}

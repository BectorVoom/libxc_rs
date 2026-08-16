//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 567/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk567(t1898: f64, t226: f64, t249: f64, t1894: f64, t252: f64, t214: f64, t1880: f64, t335: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1899 = t226 * t1898;
    let t1900 = t1899 * t249;
    let t1905 = t1894 * t252;
    let t1906 = t214 * t1905;
    let t1907 = t1880 * t1906;
    let t1932 = 1.0_f64 / t371 / t335;
    (t1899, t1900, t1905, t1906, t1907, t1932)
}

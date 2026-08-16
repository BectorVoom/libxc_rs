//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 417/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk417(t1902: f64, t218: f64, t1894: f64, t252: f64, t214: f64, t1880: f64, t235: f64, t226: f64) -> (f64, f64, f64, f64, f64) {
    let t1903 = t218 * t1902;
    let t1905 = t1894 * t252;
    let t1906 = t214 * t1905;
    let t1907 = t1880 * t1906;
    let t1909 = t235 * t1902;
    let t1911 = 0.82246703342411321825e-2_f64 * t1907 + t226 * t1909;
    (t1903, t1905, t1906, t1909, t1911)
}

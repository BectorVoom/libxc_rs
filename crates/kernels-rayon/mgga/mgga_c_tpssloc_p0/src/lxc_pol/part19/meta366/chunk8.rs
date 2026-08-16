//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1340/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1340(t2402: f64, t973: f64, t986: f64, t3010: f64, t698: f64, t10327: f64, t135: f64, t10286: f64, t2960: f64, t3016: f64, t10289: f64, t10263: f64, t2974: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42903 = t973 * t2402 * t986;
    let t42906 = t973 * t698 * t3010;
    let t42909 = t973 * t135 * t10327;
    let t42911 = t2960 * t10286;
    let t42914 = t973 * t698 * t3016;
    let t42916 = t2960 * t10289;
    let t42918 = t10263 * t2974;
    (t42903, t42906, t42909, t42911, t42914, t42916, t42918)
}

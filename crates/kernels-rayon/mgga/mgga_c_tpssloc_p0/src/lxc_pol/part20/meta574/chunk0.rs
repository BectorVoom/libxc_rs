//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2137/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2137(t3016: f64, t698: f64, t973: f64, t10289: f64, t2960: f64, t10263: f64, t2974: f64, t10348: f64, t135: f64, t10352: f64, t10232: f64, t10208: f64, t13822: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42914 = t973 * t698 * t3016;
    let t42916 = t2960 * t10289;
    let t42918 = t10263 * t2974;
    let t42925 = t973 * t135 * t10348;
    let t42936 = t2960 * t10352;
    let t42944 = t2960 * t10232;
    let t42951 = t973 * t13822 * t10208;
    (t42914, t42916, t42918, t42925, t42936, t42944, t42951)
}

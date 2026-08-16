//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1536/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1536(t10224: f64, t1592: f64, t973: f64, t2960: f64, t4528: f64, t1599: f64, t698: f64, t135: f64, t4542: f64, t13552: f64, t13550: f64, t13644: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13895 = t10224 * t1592;
    let t13896 = t973 * t13895;
    let t13907 = 0.14814814814814814814e-2_f64 * t2960 * t4528;
    let t13908 = t698 * t1599;
    let t13909 = t973 * t13908;
    let t13913 = t135 * t4542;
    let t13915 = 0.55555555555555555554e-3_f64 * t973 * t13913;
    let t13921 = 2.0_f64 / 27.0_f64 * t13552;
    let t13922 = 4.0_f64 / 9.0_f64 * t13550;
    let t13923 = 2.0_f64 / 9.0_f64 * t13644;
    (t13896, t13907, t13909, t13915, t13921, t13922, t13923)
}

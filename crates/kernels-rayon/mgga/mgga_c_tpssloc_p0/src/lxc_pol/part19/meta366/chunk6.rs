//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1338/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1338(t42308: f64, t974: f64, t344: f64, t41666: f64, t10224: f64, t2999: f64, t973: f64, t2978: f64, t698: f64, t2981: f64, t10263: f64, t2971: f64) -> (f64, f64, f64, f64, f64) {
    let t42861 = t974 * t42308;
    let t42862 = t344 * t41666;
    let t42873 = t973 * t10224 * t2999;
    let t42875 = t698 * t2978;
    let t42877 = t973 * t42875 * t2981;
    let t42889 = t10263 * t2971;
    (t42861, t42862, t42873, t42877, t42889)
}

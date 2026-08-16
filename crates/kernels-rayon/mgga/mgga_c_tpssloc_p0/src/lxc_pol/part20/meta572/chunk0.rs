//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2135/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2135(t10231: f64, t10279: f64, t973: f64, t42308: f64, t974: f64, t10224: f64, t2999: f64, t2978: f64, t698: f64, t2981: f64, t10263: f64, t2971: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42858 = t973 * t10231 * t10279;
    let t42861 = t974 * t42308;
    let t42873 = t973 * t10224 * t2999;
    let t42875 = t698 * t2978;
    let t42877 = t973 * t42875 * t2981;
    let t42889 = t10263 * t2971;
    (t42858, t42861, t42873, t42875, t42877, t42889)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1091/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1091(t11915: f64, t22348: f64, t1734: f64, t1932: f64, t475: f64, t6260: f64, t11883: f64, t11889: f64, t1751: f64, t6224: f64, t3612: f64, t6218: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22349 = t22348 * t11915;
    let t22354 = t1932 * t1734 * t475;
    let t22355 = t6260 * t22354;
    let t22358 = t22348 * t11883;
    let t22361 = t22348 * t11889;
    let t22364 = t1751 * t6224;
    let t22365 = t22364 * t3612;
    let t22368 = t3612 * t6218;
    (t22349, t22355, t22358, t22361, t22364, t22365, t22368)
}

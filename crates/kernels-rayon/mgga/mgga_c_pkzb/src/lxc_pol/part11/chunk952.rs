//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 952/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk952(t10388: f64, t330: f64, t328: f64, t3340: f64, t987: f64, t3337: f64, t995: f64, t3356: f64, t3314: f64, t973: f64, t4794: f64, t2489: f64, t3318: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10389 = t330 * t10388;
    let t10390 = t328 * t10389;
    let t10405 = t987 * t3340;
    let t10408 = t3337 * t995;
    let t10411 = t987 * t3356;
    let t10414 = t3314 * t973;
    let t10415 = t4794 * t10414;
    let t10418 = t2489 * t3318;
    (t10389, t10390, t10405, t10408, t10411, t10414, t10415, t10418)
}

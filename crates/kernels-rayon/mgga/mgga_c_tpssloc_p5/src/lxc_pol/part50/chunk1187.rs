//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1187/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1187(t112899: f64, t22986: f64, t25192: f64, t112760: f64, t112719: f64, t1484: f64, t23270: f64, t2717: f64, t7537: f64, t1888: f64, t865: f64, t30634: f64, t86873: f64) -> (f64, f64, f64, f64, f64) {
    let t118802 = 0.3289868133696452873e-1_f64 * t22986 * t112899 * t25192;
    let t118810 = 0.38381794893125283518e-1_f64 * t112760;
    let t118814 = 0.3289868133696452873e-1_f64 * t22986 * t23270 * t112719 * t1484;
    let t118821 = t2717 * t7537;
    let t118825 = 0.3289868133696452873e-1_f64 * t1888 * t23270 * t118821 * t865;
    let t118828 = 0.3289868133696452873e-1_f64 * t1888 * t86873 * t30634;
    (t118802, t118810, t118814, t118825, t118828)
}

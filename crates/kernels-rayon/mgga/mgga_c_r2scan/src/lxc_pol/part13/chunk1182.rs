//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1182/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1182(t29418: f64, t3293: f64, t132: f64, t537: f64, t7322: f64, t1575: f64, t25826: f64, t3342: f64, t571: f64, t1054: f64, t2139: f64, t7356: f64) -> (f64, f64, f64, f64) {
    let t40194 = t3293 * t29418;
    let t40195 = t132 * t537;
    let t40197 = t40194 * t40195 * t7322;
    let t40201 = t571 * t1575 * t3342 * t25826;
    let t40204 = t2139 * t1054 * t7356;
    (t40195, t40197, t40201, t40204)
}

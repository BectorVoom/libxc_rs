//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 803/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk803(t471: f64, t7088: f64, t97: f64, t7007: f64, t86: f64, t2484: f64, t406: f64, t410: f64, t166: f64, t2483: f64, t607: f64, t1783: f64, t898: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7090 = t97 * t471 * t7088;
    let t7091 = 3.0_f64 * t7090;
    let t7092 = t7007 * t86;
    let t7093 = 0.19751673498613801407e-1_f64 * t7092;
    let t7094 = t406 * t2484;
    let t7095 = 8.0_f64 * t7094;
    let t7096 = t410 * t2484;
    let t7097 = 8.0_f64 * t7096;
    let t7098 = t7007 * t166;
    let t7101 = t2483 * t607;
    let t7104 = t898 * t1783;
    (t7091, t7093, t7095, t7097, t7098, t7101, t7104)
}

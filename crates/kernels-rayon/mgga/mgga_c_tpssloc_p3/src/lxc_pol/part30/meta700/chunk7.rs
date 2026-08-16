//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2260/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2260(t23168: f64, t28277: f64, t28295: f64, t6547: f64, t6562: f64, t7488: f64, t86893: f64, t28439: f64, t28268: f64, t81591: f64, t17049: f64, t1880: f64, t6553: f64, t6571: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98921 = t23168 * t28277;
    let t98923 = t6547 * t28295;
    let t98927 = t6562 * t86893 * t7488;
    let t98932 = t6547 * t28439;
    let t98941 = t81591 * t28268;
    let t98945 = t1880 * t6553 * t6571 * t17049;
    (t98921, t98923, t98927, t98932, t98941, t98945)
}

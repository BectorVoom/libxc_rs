//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2928/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2928(t53251: f64, t53272: f64, t11223: f64, t1678: f64, t16163: f64, t3169: f64, t1041: f64, t11262: f64, t4868: f64, t1058: f64, t15859: f64, t3201: f64, t4794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53273 = t53251 + t53272;
    let t53281 = t11223 * t1678;
    let t53290 = t3169 * t16163;
    let t53293 = t1041 * t11262 * t4868;
    let t53294 = 0.14291339372689912324e-3_f64 * t53293;
    let t53298 = t15859 * t1058;
    let t53300 = t4794 * t3201;
    (t53273, t53281, t53290, t53294, t53298, t53300)
}

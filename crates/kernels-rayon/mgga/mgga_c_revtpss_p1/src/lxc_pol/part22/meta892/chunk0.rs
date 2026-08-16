//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3080/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3080(t1678: f64, t3043: f64, t3259: f64, t4746: f64, t15885: f64, t993: f64, t378: f64, t11223: f64, t16163: f64, t3169: f64, t1041: f64, t11262: f64, t4868: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53180 = t3043 * t1678;
    let t53208 = t4746 * t3259;
    let t53222 = t15885 * t993;
    let t53223 = t53222 * t378;
    let t53281 = t11223 * t1678;
    let t53290 = t3169 * t16163;
    let t53293 = t1041 * t11262 * t4868;
    (t53180, t53208, t53222, t53223, t53281, t53290, t53293)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3104/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3104(t11933: f64, t16035: f64, t11774: f64, t127: f64, t15585: f64, t4872: f64, t16226: f64, t16229: f64, t53405: f64, t3230: f64, t4857: f64, t11817: f64, t4858: f64) -> (f64, f64, f64, f64, f64) {
    let t54324 = t11933 * t16035;
    let t54341 = t11774 * t127 * t4872 * t15585;
    let t54348 = t16226 * t53405 * t16229;
    let t54384 = t4857 * t3230;
    let t54387 = t4858 * t11817;
    (t54324, t54341, t54348, t54384, t54387)
}

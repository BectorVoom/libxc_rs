//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3107/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3107(t12021: f64, t4820: f64, t11922: f64, t15921: f64, t3115: f64, t1086: f64, t15669: f64, t3090: f64, t43347: f64, t53668: f64, t16163: f64, t3124: f64) -> (f64, f64, f64, f64, f64) {
    let t54490 = t12021 * t4820;
    let t54497 = t3115 * t11922 * t15921;
    let t54500 = t15669 * t1086 * t3090;
    let t54509 = t43347 * t53668;
    let t54521 = t3124 * t16163;
    (t54490, t54497, t54500, t54509, t54521)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3114/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3114(t12078: f64, t53740: f64, t12047: f64, t16138: f64, t372: f64, t16158: f64, t3106: f64, t12003: f64, t1659: f64, t11648: f64, t4879: f64, t1063: f64, t15790: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54801 = t12078 * t53740;
    let t54811 = t12047 * t53740;
    let t54818 = t372 * t16138;
    let t54836 = t3106 * t16158;
    let t54838 = t1659 * t12003;
    let t54841 = t4879 * t11648;
    let t54849 = t1063 * t3172 * t15790;
    (t54801, t54811, t54818, t54836, t54838, t54841, t54849)
}

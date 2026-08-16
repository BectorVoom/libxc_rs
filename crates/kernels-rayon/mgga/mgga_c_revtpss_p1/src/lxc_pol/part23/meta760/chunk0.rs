//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2554/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2554(t15731: f64, t3169: f64, t12078: f64, t53740: f64, t12047: f64, t16138: f64, t372: f64, t11671: f64, t15925: f64, t1063: f64, t11986: f64, t247: f64, t4583: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54733 = t3169 * t15731;
    let t54801 = t12078 * t53740;
    let t54811 = t12047 * t53740;
    let t54818 = t372 * t16138;
    let t54916 = t15925 * t11671;
    let t54943 = t1063 * t247 * t11986 * t4583;
    (t54733, t54801, t54811, t54818, t54916, t54943)
}

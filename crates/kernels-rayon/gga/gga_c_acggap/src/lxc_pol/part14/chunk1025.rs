//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1025/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1025(t36085: f64, t7839: f64, t8908: f64, t8912: f64, t8970: f64, t1181: f64, t31567: f64, t36019: f64, t599: f64, t1992: f64, t7585: f64, t7586: f64, t8960: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36086 = 0.10718504529517434243e-2_f64 * t36085;
    let t36087 = t7839 * t8908;
    let t36088 = 0.42874018118069736972e-3_f64 * t36087;
    let t36089 = t7839 * t8912;
    let t36090 = 0.21437009059034868486e-3_f64 * t36089;
    let t36096 = t7839 * t8970;
    let t36097 = 0.31448092289604152068e-3_f64 * t36096;
    let t36115 = t31567 * t1181 * t599 * t36019;
    let t36119 = t7585 * t7586 * t1992 * t8960;
    (t36086, t36088, t36090, t36097, t36115, t36119)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2131/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2131(t2926: f64, t4631: f64, t934: f64, t2924: f64, t2918: f64, t4635: f64, t11387: f64, t1609: f64, t2875: f64, t11385: f64, t4644: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15389 = t4631 * t2926;
    let t15390 = t15389 * t934;
    let t15392 = 0.32163958997385070134e2_f64 * t2924 * t15390;
    let t15393 = t4635 * t2918;
    let t15395 = 0.16081979498692535067e2_f64 * t2924 * t15393;
    let t15396 = t1609 * t11387;
    let t15397 = t15396 * t2875;
    let t15399 = 0.51726012919273400301e3_f64 * t11385 * t15397;
    let t15400 = t4644 * t945;
    (t15390, t15392, t15393, t15395, t15397, t15399, t15400)
}

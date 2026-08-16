//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2662/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2662(t11921: f64, t15716: f64, t19456: f64, t247: f64, t19696: f64, t3168: f64, t15830: f64, t4817: f64, t1063: f64, t11986: f64, t6100: f64, t20054: f64, t3106: f64) -> (f64, f64, f64, f64, f64) {
    let t65298 = t15716 * t247 * t11921 * t19456;
    let t65342 = t19696 * t3168;
    let t65347 = t15830 * t4817;
    let t65357 = t1063 * t247 * t11986 * t6100;
    let t65359 = t3106 * t20054;
    (t65298, t65342, t65347, t65357, t65359)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1033/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1033(t12752: f64, t1545: f64, t1008: f64, t4724: f64, t14173: f64, t4916: f64, t3391: f64, t4680: f64, t4915: f64, t1111: f64, t1181: f64, t15995: f64) -> (f64, f64, f64, f64, f64) {
    let t17708 = t12752 * t1545;
    let t17710 = t1008 * t4724;
    let t17718 = t14173 * t4916;
    let t17721 = t3391 * t4680 * t4915;
    let t17725 = t3391 * t1181 * t15995 * t1111;
    (t17708, t17710, t17718, t17721, t17725)
}

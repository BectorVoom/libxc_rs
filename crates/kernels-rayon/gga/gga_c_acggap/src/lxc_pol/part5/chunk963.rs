//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 963/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk963(t1181: f64, t15407: f64, t3456: f64, t535: f64, t1165: f64, t14575: f64, t3194: f64, t530: f64, t14050: f64, t4971: f64, t3379: f64, t4975: f64) -> (f64, f64, f64, f64) {
    let t15410 = t3456 * t1181 * t535 * t15407;
    let t15429 = t3194 * t1165 * t530 * t14575;
    let t15431 = t14050 * t4971;
    let t15469 = t3379 * t4975;
    (t15410, t15429, t15431, t15469)
}

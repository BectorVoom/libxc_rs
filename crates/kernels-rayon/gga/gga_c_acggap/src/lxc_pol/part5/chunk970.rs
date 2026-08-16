//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 970/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk970(t3375: f64, t4372: f64, t1163: f64, t1165: f64, t4162: f64, t4289: f64, t157: f64, t406: f64, t864: f64) -> (f64, f64, f64) {
    let t15750 = t3375 * t4372;
    let t15754 = t1163 * t1165 * t4289 * t4162;
    let t15758 = t864 * t406 * t157;
    (t15750, t15754, t15758)
}

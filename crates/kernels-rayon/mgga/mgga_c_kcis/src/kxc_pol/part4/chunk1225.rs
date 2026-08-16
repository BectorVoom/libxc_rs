//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1225/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1225(t6019: f64, t1498: f64, t1464: f64, t11783: f64, t2002: f64, t3954: f64, t5632: f64, t1468: f64, t4124: f64, t4123: f64, t3734: f64, t5633: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15808 = t6019 * sigma2;
    let t15809 = t15808 * t1498;
    let t15810 = t1464 * t15809;
    let t15812 = t11783 * t2002;
    let t15813 = t1464 * t15812;
    let t15815 = t5632 * t3954;
    let t15816 = t1468 * t15815;
    let t15817 = t1464 * t15816;
    let t15819 = t5632 * t4124;
    let t15820 = t4123 * t15819;
    let t15821 = t1464 * t15820;
    let t15823 = t3734 * t5633;
    (t15808, t15810, t15813, t15817, t15821, t15823)
}

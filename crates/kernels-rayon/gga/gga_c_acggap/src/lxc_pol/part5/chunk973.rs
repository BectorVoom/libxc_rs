//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 973/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk973(t1032: f64, t4720: f64, t4523: f64, t1008: f64, t5108: f64, t14106: f64, t542: f64, t13957: f64, t532: f64, t4396: f64, t5138: f64, t5143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15830 = t1032 * t4720;
    let t15832 = t1032 * t4523;
    let t15841 = t1008 * t5108;
    let t15849 = t14106 * t542;
    let t15851 = t13957 * t542;
    let t15853 = t13957 * t532;
    let t15855 = t4396 * t5138;
    let t15871 = t4396 * t5143;
    (t15830, t15832, t15841, t15849, t15851, t15853, t15855, t15871)
}

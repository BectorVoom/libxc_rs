//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 975/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk975(t3431: f64, t4975: f64, t5157: f64, t13957: f64, t537: f64, t4878: f64, t997: f64, t4853: f64, t4849: f64, t1581: f64, t3237: f64, t3379: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15920 = t3431 * t4975;
    let t15922 = t3431 * t5157;
    let t15930 = t13957 * t537;
    let t15932 = t997 * t4878;
    let t15934 = t997 * t4853;
    let t15936 = t997 * t4849;
    let t15938 = t3237 * t1581;
    let t15945 = t3379 * t5157;
    (t15920, t15922, t15930, t15932, t15934, t15936, t15938, t15945)
}

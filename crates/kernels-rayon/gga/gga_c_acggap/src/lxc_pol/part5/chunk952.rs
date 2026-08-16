//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 952/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk952(t1603: f64, t310: f64, t464: f64, t1620: f64, t3892: f64, t3919: f64, t5371: f64, t3915: f64, t12203: f64, t3044: f64, t448: f64, t556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15151 = t310 * t1603;
    let t15152 = t15151 * t464;
    let t15154 = t3892 * t1620;
    let t15156 = t5371 * t3919;
    let t15159 = t5371 * t3915;
    let t15164 = t12203 * t448 * t556 * t3044;
    (t15151, t15152, t15154, t15156, t15159, t15164)
}

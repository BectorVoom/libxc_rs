//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 954/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk954(t1222: f64, t4137: f64, t3882: f64, t5384: f64, t5385: f64, t1620: f64, t3896: f64, t1308: f64, t3912: f64, t1614: f64, t3901: f64, t3930: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15196 = t4137 * t1222;
    let t15199 = t5384 * t5385 * t3882;
    let t15201 = t3896 * t1620;
    let t15204 = t1308 * t3912;
    let t15206 = t3901 * t1614;
    let t15208 = t3930 * t1614;
    (t15196, t15199, t15201, t15204, t15206, t15208)
}

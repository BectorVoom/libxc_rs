//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1042/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1042(t3379: f64, t5277: f64, t5281: f64, t1181: f64, t15758: f64, t3451: f64, t535: f64, t16325: f64, t4282: f64, t530: f64, t1165: f64, t3456: f64, t4241: f64, t4289: f64) -> (f64, f64, f64, f64, f64) {
    let t18017 = t3379 * t5277;
    let t18019 = t3379 * t5281;
    let t18027 = t3451 * t1181 * t535 * t15758;
    let t18031 = t4282 * t1181 * t530 * t16325;
    let t18035 = t3456 * t1165 * t4289 * t4241;
    (t18017, t18019, t18027, t18031, t18035)
}

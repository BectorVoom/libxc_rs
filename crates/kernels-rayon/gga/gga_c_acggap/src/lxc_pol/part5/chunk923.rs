//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 923/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk923(t14106: f64, t425: f64, t431: f64, t438: f64, t1195: f64, t3228: f64, t1200: f64, t1205: f64, t3770: f64, t993: f64, t1032: f64, t3697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14107 = t14106 * t425;
    let t14109 = t14106 * t431;
    let t14111 = t14106 * t438;
    let t14113 = t3228 * t1195;
    let t14115 = t3228 * t1200;
    let t14117 = t3228 * t1205;
    let t14120 = 0.12004725073059526352e-1_f64 * t3770 * t993;
    let t14122 = 0.40015750243531754508e-2_f64 * t1032 * t3697;
    (t14107, t14109, t14111, t14113, t14115, t14117, t14120, t14122)
}

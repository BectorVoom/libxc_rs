//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1133/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1133(t292: f64, t43789: f64, t1771: f64, t2783: f64, t3051: f64, t854: f64, t10603: f64, t13682: f64, t13688: f64, t15042: f64, t15047: f64, t192: f64, t19714: f64, t2771: f64, t2781: f64, t43386: f64, t43397: f64, t43414: f64, t43513: f64, t43553: f64, t43563: f64, t43568: f64, t43574: f64, t43578: f64, t462: f64, t824: f64, t852: f64, t92: f64) -> (f64, f64) {
    let t293 = 0.1e-59_f64 < t292;
    let t43790 = piecewise3(t293, t43789, 0.0_f64);
    let t43794 = t1771 * t2783;
    let t43796 = t3051 * t854;
    let t43798 = 8.0_f64 / 3.0_f64 * t13682 * t15042 * t43553 + 4.0_f64 / 3.0_f64 * t462 * t2771 * t43414 + 2.0_f64 * t462 * t2771 * t43397 - 4.0_f64 / 3.0_f64 * t43563 - 8.0_f64 * t13688 * t15047 * t43553 - 8.0_f64 * t13688 * t19714 * t43568 * t824 + t43574 + 8.0_f64 * t462 * t10603 * t43386 + 8.0_f64 / 3.0_f64 * t43578 + 6.0_f64 * t92 * t192 * t2781 * t43513 - t92 * t192 * t852 * t43790 + 16.0_f64 / 3.0_f64 * t43794 + 112.0_f64 / 27.0_f64 * t43796;
    (t43790, t43798)
}

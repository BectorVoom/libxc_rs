//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 989/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk989(t1173: f64, t7440: f64, t24412: f64, t27924: f64, t10052: f64, t1131: f64, t1403: f64, t140605: f64, t193: f64, t24191: f64, t27953: f64, t27965: f64, t27974: f64, t27991: f64, t33243: f64, t33253: f64, t33568: f64, t35737: f64, t6002: f64, t6008: f64, t6192: f64, t6754: f64, t684: f64, t6945: f64, t713: f64, t7437: f64, t766: f64, t9770: f64) -> (f64, f64) {
    let t149832 = t7440 * t1173;
    let t149837 = t24412 * t27924;
    let t149843 = -t140605 / 3.0_f64 - t7437 * t27965 / 3.0_f64 + t1403 * t193 * t33243 * t27974 - t33568 * t6754 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1403 * t193 * t6008 * t6192 * t1131 - t1403 * t193 * t33253 * t27974 / 3.0_f64 - 24.0_f64 * t10052 * t35737 * t766 - 2.0_f64 / 3.0_f64 * t1403 * t193 * t6008 * t6945 * t713 - t7437 * t27953 / 3.0_f64 + t6002 * t9770 * t149832 * t684 / 9.0_f64 + 8.0_f64 * t149837 - 2.0_f64 / 3.0_f64 * t1403 * t193 * t24191 * t27991;
    (t149837, t149843)
}

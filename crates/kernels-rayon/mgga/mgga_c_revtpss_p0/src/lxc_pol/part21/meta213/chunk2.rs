//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1283/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1283(t1149: f64, t5108: f64, t3433: f64, t3358: f64, t3439: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t1160: f64, t1737: f64, t1168: f64, t1745: f64) -> (f64, f64, f64, f64, f64) {
    let t5109 = t5108 * t1149;
    let t5111 = 0.16081979498692535067e2_f64 * t3433 * t5109;
    let t5117 = t3439 - 0.57077777777777777777e-2_f64 * t3358 - 0.57077777777777777777e-2_f64 * t5044 - 0.11415555555555555555e-1_f64 * t5049 + 0.34246666666666666666e-1_f64 * t5054 + 0.17123333333333333333e-1_f64 * t5058;
    let t5120 = t1737 * t1160;
    let t5125 = t1745 * t1168;
    (t5109, t5111, t5117, t5120, t5125)
}

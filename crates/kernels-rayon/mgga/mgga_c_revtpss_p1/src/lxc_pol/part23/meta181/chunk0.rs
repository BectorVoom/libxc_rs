//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1078/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1078(t1150: f64, t5104: f64, t1131: f64, t1732: f64, t3435: f64, t1149: f64, t3433: f64, t3358: f64, t3439: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5105 = t5104 * t1150;
    let t5107 = 1.0_f64 * t1131 * t5105;
    let t5108 = t1732 * t3435;
    let t5109 = t5108 * t1149;
    let t5111 = 0.16081979498692535067e2_f64 * t3433 * t5109;
    let t5117 = t3439 - 0.57077777777777777777e-2_f64 * t3358 - 0.57077777777777777777e-2_f64 * t5044 - 0.11415555555555555555e-1_f64 * t5049 + 0.34246666666666666666e-1_f64 * t5054 + 0.17123333333333333333e-1_f64 * t5058;
    (t5105, t5107, t5108, t5109, t5111, t5117)
}

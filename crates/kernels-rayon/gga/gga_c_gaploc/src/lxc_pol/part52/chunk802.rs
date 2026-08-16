//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 802/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk802(t41060: f64, t41071: f64, t10040: f64, t25198: f64, t13055: f64, t5640: f64, t13058: f64, t1991: f64, t20671: f64, t28309: f64, t33601: f64, t33565: f64, t7372: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43602 = 0.25561950635947166451e0_f64 * t41060;
    let t43604 = 0.25561950635947166451e0_f64 * t41071;
    let t43646 = t25198 * t10040;
    let t43652 = t5640 * t13055;
    let t43657 = t1991 * t13058;
    let t43660 = t28309 * t20671 * t33601;
    let t43679 = t33565 * t7372;
    (t43602, t43604, t43646, t43652, t43657, t43660, t43679)
}

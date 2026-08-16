//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1146/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1146(t174: f64, t507: f64, t435: f64, t495: f64, t930: f64, t1298: f64, t407: f64, t1150: f64, t1165: f64, t1173: f64, t1460: f64, t1524: f64, t1532: f64, t15690: f64, t15695: f64, t15814: f64, t1849: f64, t301: f64, t372: f64, t4255: f64, t4256: f64, t4257: f64, t4261: f64, t4263: f64, t4593: f64, t5164: f64, t5544: f64, t5549: f64, t5651: f64, t5693: f64, t839: f64, t8927: f64, t922: f64) -> (f64, f64) {
    let t20555 = t507 * t174;
    let t20559 = t507 * t435;
    let t20590 = t930 * t495;
    let t20595 = t407 * t1298;
    let t20600 = -t1150 * t4593 * t5164 / 8.0_f64 - t4255 * t20555 * t4257 / 4.0_f64 - t4261 * t20559 * t4263 / 6.0_f64 - t15690 * t8927 * t1460 * t1524 / 4.0_f64 - t4255 * t15695 * t5693 / 4.0_f64 - t4255 * t4256 * t5544 * t301 / 4.0_f64 - t4255 * t4256 * t5549 * t301 / 4.0_f64 - t4255 * t4256 * t1849 * t839 / 8.0_f64 - t4255 * t4256 * t5651 * t372 / 8.0_f64 + t15814 * t4256 * t1849 * t922 / 2.0_f64 + 0.17149607247227894789e-2_f64 * t1173 * t1165 * t1532 * t20590 + 0.34299214494455789578e-2_f64 * t1173 * t1165 * t1532 * t20595;
    (t20595, t20600)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1318/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1318(t13251: f64, t1510: f64, t16836: f64, t16839: f64, t20756: f64, t20852: f64, t20882: f64, t20891: f64, t20983: f64, t232: f64, t2632: f64, t2643: f64, t2645: f64, t41467: f64, t4178: f64, t4180: f64, t4181: f64, t5544: f64, t5587: f64, t5593: f64, t58574: f64, t58576: f64, t58642: f64, t58811: f64, t67620: f64, t67852: f64, t67854: f64) -> f64 {
    let t76227 = t58811 * t5587 / 256.0_f64 - t4178 * t2645 * t16839 * t2632 * t5544 / 64.0_f64 + t4178 * t4180 * t4181 * t2632 * t20852 / 384.0_f64 - t16836 * t20983 / 32.0_f64 + t58642 * t5593 / 64.0_f64 + 595.0_f64 / 576.0_f64 * t58574 - 119.0_f64 / 1152.0_f64 * t58576 - t2643 * t4180 * t67620 * t1510 / 768.0_f64 + 5.0_f64 / 32.0_f64 * t2643 * t41467 * t4181 * t232 * t20756 + t13251 * t20882 / 64.0_f64 - t13251 * t20891 / 256.0_f64 + 7.0_f64 / 384.0_f64 * t67852 + 7.0_f64 / 384.0_f64 * t67854;
    t76227
}

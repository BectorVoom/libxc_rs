//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1316/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1316(t119: f64, t13251: f64, t16836: f64, t16839: f64, t20885: f64, t20974: f64, t20978: f64, t20986: f64, t20988: f64, t210: f64, t2571: f64, t2643: f64, t2645: f64, t2701: f64, t41161: f64, t4178: f64, t4180: f64, t46546: f64, t5591: f64, t58421: f64, t67620: f64, t67660: f64, t67675: f64, t76056: f64, t76063: f64, t820: f64, t843: f64) -> f64 {
    let t76167 = t16836 * t20988 / 128.0_f64 + 455.0_f64 / 162.0_f64 * t46546 + 119.0_f64 / 288.0_f64 * t58421 + 3.0_f64 / 256.0_f64 * t4178 * t4180 * t16839 * t20986 - 5.0_f64 / 64.0_f64 * t13251 * t20974 + t13251 * t20978 / 64.0_f64 + t2643 * t2645 * t16839 * t20885 / 128.0_f64 + t2643 * t2645 * t67620 * t5591 / 192.0_f64 + 5.0_f64 / 4.0_f64 * t41161 * t210 * t119 * t76056 + 3.0_f64 / 16.0_f64 * t2571 * t210 * t119 * t76063 + 7.0_f64 / 192.0_f64 * t67660 - 35.0_f64 / 96.0_f64 * t67675 + 5.0_f64 / 256.0_f64 * t843 * t2701 * t820 * t76063;
    t76167
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1216/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1216(t2617: f64, t9637: f64, t2649: f64, t2691: f64, t812: f64, t815: f64, t10003: f64, t119: f64, t13222: f64, t13254: f64, t210: f64, t2633: f64, t2643: f64, t2647: f64, t40848: f64, t41078: f64, t41084: f64, t41086: f64, t41088: f64, t41090: f64, t41096: f64, t4178: f64, t4180: f64, t4182: f64, t787: f64, t9621: f64, t9629: f64, t9642: f64, t9646: f64, t9647: f64) -> f64 {
    let t41107 = t2617 * t9637;
    let t41108 = t41107 * t2649;
    let t41115 = t812 * t815 * t2691;
    let t41116 = t41115 * t2649;
    let t41120 = t2643 * t13222 * t41078 * t2647 / 64.0_f64 + 455.0_f64 / 162.0_f64 * t41084 - 35.0_f64 / 36.0_f64 * t41086 + 7.0_f64 / 36.0_f64 * t41088 - t4178 * t13222 * t4182 * t41090 / 32.0_f64 + t41096 - 5.0_f64 / 128.0_f64 * t2643 * t9646 * t9621 * t9647 + t9642 * t10003 / 64.0_f64 + 3.0_f64 / 256.0_f64 * t4178 * t4180 * t9621 * t2633 - 7.0_f64 / 48.0_f64 * t41108 - t787 * t210 * t119 * t40848 / 48.0_f64 + 119.0_f64 / 288.0_f64 * t41116 - t13254 * t9629 / 32.0_f64;
    t41120
}

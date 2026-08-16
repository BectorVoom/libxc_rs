//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2802/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2802(t1509: f64, t5911: f64, t105: f64, t108: f64, t13496: f64, t1507: f64, t2: f64, t21861: f64, t21865: f64, t21869: f64, t21872: f64, t21873: f64, t2255: f64, t22617: f64, t22624: f64, t2357: f64, t4279: f64, t4284: f64, t46212: f64, t49787: f64, t580: f64, t5902: f64, t5907: f64, t661: f64, t75625: f64, t75879: f64) -> f64 {
    let t75906 = t1509 * t5911;
    let t75924 = -200.0_f64 / 9.0_f64 * t5902 * t4284 + 50.0_f64 / 27.0_f64 * t1507 * t21861 + 100.0_f64 / 9.0_f64 * t75625 * t21865 - 50.0_f64 / 9.0_f64 * t1507 * t21869 - 25.0_f64 / 3.0_f64 * t1507 * t21873 + 40.0_f64 / 81.0_f64 * t105 * t46212 * t22617 * t661 + 10.0_f64 / 9.0_f64 * t49787 * t5907 * t2 * t580 - 10.0_f64 / 9.0_f64 * t49787 * t75906 * t661 - 10.0_f64 / 3.0_f64 * t13496 * t2255 * t5911 + 10.0_f64 / 3.0_f64 * t105 * t4279 * t21872 + 10.0_f64 / 9.0_f64 * t105 * t2357 * t22624 * t661 - 5.0_f64 / 3.0_f64 * t105 * t108 * t75879;
    t75924
}

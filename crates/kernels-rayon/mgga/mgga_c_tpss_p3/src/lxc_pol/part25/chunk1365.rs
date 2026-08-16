//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1365/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1365(t21658: f64, t2436: f64, t1288: f64, t13334: f64, t1692: f64, t1812: f64, t18728: f64, t19678: f64, t19810: f64, t20417: f64, t20510: f64, t20526: f64, t21583: f64, t2439: f64, t36547: f64, t5591: f64, t6153: f64, t66281: f64, t66317: f64, t69796: f64, t69817: f64, t69820: f64, t69858: f64, t69882: f64, t69887: f64, t70221: f64) -> (f64, f64) {
    let t72265 = t21658 * t2436;
    let t72277 = 3.0_f64 / 2.0_f64 * t2439 * t1812 * t69887 - 3.0_f64 * t66317 * t19678 - t1692 * t66281 * t6153 - 3.0_f64 * t66317 * t19810 + 3.0_f64 * t36547 * t21583 - 3.0_f64 * t20417 * t69820 + t1692 * t20510 * t1288 + t1692 * t1812 * t13334 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t70221 - t1692 * t72265 * t5591 / 2.0_f64 + 6.0_f64 * t20417 * t69817 - 3.0_f64 * t18728 * t69796 + 2.0_f64 * t20526 * t69882 - 3.0_f64 / 2.0_f64 * t18728 * t69858;
    (t72265, t72277)
}

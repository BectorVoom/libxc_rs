//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2847/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2847(t141: f64, t41294: f64, t51856: f64, t51865: f64, t930: f64, t51869: f64, t51861: f64, t11150: f64, t2251: f64, t4186: f64, t2908: f64, t10356: f64, t1469: f64, t41270: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51981 = t141 * t41294 * t51856;
    let t51984 = t141 * t930 * t51865;
    let t51987 = t141 * t930 * t51869;
    let t51990 = t141 * t930 * t51861;
    let t51993 = t11150 * t4186 * t2251;
    let t51995 = t141 * t2908 * t51993;
    let t51998 = t41270 * t1469 * t10356;
    (t51981, t51984, t51987, t51990, t51993, t51995, t51998)
}

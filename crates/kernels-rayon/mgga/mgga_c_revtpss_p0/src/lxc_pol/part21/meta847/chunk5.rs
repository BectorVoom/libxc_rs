//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3180/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3180(t1131: f64, t1150: f64, t58491: f64, t58504: f64, t58518: f64, t58531: f64, t58545: f64, t58558: f64, t58572: f64, t58585: f64, t12470: f64, t1744: f64) -> (f64, f64) {
    let t58591 = 1.0_f64 * t1131 * (t58491 + t58504 + t58518 + t58531 + t58545 + t58558 + t58572 + t58585) * t1150;
    let t58592 = t12470 * t1744;
    (t58591, t58592)
}

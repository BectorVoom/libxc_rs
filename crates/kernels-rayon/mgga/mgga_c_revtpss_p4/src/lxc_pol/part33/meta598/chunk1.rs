//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2020/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2020(t94590: f64, t94771: f64, t25304: f64, t25949: f64, t25946: f64, t25878: f64, t94661: f64, t7246: f64, t9692: f64, t1419: f64, t7063: f64, t25898: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94772 = t94771 * t94590;
    let t94776 = t25304 * t25949;
    let t94777 = t94776 * t25946;
    let t94779 = t25878 * t94661;
    let t94784 = 0.30356481678079769392e-1_f64 * t7246 * t9692;
    let t94801 = t7063 * t1419;
    let t94802 = t94801 * t25898;
    (t94772, t94777, t94779, t94784, t94801, t94802)
}

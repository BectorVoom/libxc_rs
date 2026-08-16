//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1588/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1588(t1904: f64, t22445: f64, t689: f64, t22974: f64, t47603: f64, t686: f64, t72: f64, t213: f64, t22964: f64, t13729: f64, t2782: f64, t556: f64, t6918: f64) -> (f64, f64, f64, f64) {
    let t86682 = t689 * t22445 * t1904;
    let t86699 = t47603 * t22974 * t72 * t686;
    let t86701 = t213 * t22964;
    let t86712 = t2782 * t556 * t13729 * t6918;
    (t86682, t86699, t86701, t86712)
}

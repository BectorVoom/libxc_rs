//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2782/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2782(t14110: f64, t49471: f64, t136: f64, t2457: f64, t47480: f64, t6895: f64, t22414: f64, t686: f64, t72: f64, t9680: f64, t22386: f64, t3915: f64) -> (f64, f64, f64, f64) {
    let t74763 = t49471 * t14110;
    let t74770 = t47480 * t6895 * t136 * t2457;
    let t74782 = t9680 * t22414 * t72 * t686;
    let t74794 = t3915 * t22386 * t72 * t686;
    (t74763, t74770, t74782, t74794)
}

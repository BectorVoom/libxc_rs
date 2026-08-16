//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 955/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk955(t117: f64, t33374: f64, t32172: f64, t32174: f64, t32176: f64, t32178: f64, t32828: f64, t32830: f64, t32832: f64, t33346: f64, t670: f64, t8564: f64) -> (f64, f64) {
    let t33375 = t33374 * t117;
    let t33381 = 2.0_f64 * t33346 * t670 + t32172 + t32174 + t32176 + t32178 + 4.0_f64 * t32828 + 4.0_f64 * t32830 + 4.0_f64 * t32832 + t33375 + t8564;
    (t33375, t33381)
}

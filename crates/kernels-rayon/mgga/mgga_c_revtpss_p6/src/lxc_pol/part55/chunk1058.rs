//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1058/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1058(t7536: f64, t8717: f64, t2014: f64, t1936: f64, t26399: f64, t28658: f64, t7002: f64, t7359: f64, t2055: f64, t32392: f64, t93: f64, t7373: f64, t8692: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32636 = t7536 * t8717;
    let t32637 = t2014 * t32636;
    let t32642 = 2.0_f64 * t26399 * t1936;
    let t32644 = 2.0_f64 * t28658 * t1936;
    let t32646 = 2.0_f64 * t7359 * t7002;
    let t32654 = 2.0_f64 * t32392 * t2055;
    let t32655 = t93 * t7002;
    let t32657 = 2.0_f64 * t32655 * t2055;
    let t32659 = 2.0_f64 * t8692 * t7373;
    (t32636, t32637, t32642, t32644, t32646, t32654, t32655, t32657, t32659)
}

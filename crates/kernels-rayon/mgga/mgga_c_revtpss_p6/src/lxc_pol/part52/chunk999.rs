//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 999/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk999(t212: f64, t8085: f64, t1358: f64, t689: f64, t2097: f64, t543: f64, t5658: f64, t7301: f64, t786: f64, t8086: f64, t1364: f64, t5774: f64) -> (f64, f64, f64, f64) {
    let t28824 = t212 * t8085;
    let t28825 = t28824 * t1358;
    let t28826 = t689 * t28825;
    let t28829 = t2097 * t5658 * t543;
    let t28830 = t7301 * t28829;
    let t28837 = t786 * t8086;
    let t28838 = t28837 * t1364;
    let t28840 = t2097 * t5774;
    (t28826, t28830, t28838, t28840)
}

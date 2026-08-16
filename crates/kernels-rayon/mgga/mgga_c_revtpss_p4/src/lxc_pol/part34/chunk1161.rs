//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1161/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1161(t225: f64, t29807: f64, t385: f64, t1982: f64, t6343: f64, t1695: f64, t7821: f64, t7160: f64, t1089: f64, t1668: f64, t27604: f64, t6299: f64, t7168: f64) -> (f64, f64, f64, f64, f64) {
    let t29809 = t29807 * t225 * t385;
    let t29812 = t1982 * t6343;
    let t29817 = t7821 * t1695;
    let t29818 = t7160 * t29817;
    let t29822 = t27604 * t1668 * t1089;
    let t29826 = t7168 * t6299 * t1089;
    (t29809, t29812, t29818, t29822, t29826)
}

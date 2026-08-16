//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3120/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3120(t11774: f64, t16103: f64, t53405: f64, t16170: f64, t372: f64, t12116: f64, t15688: f64, t11773: f64, t15925: f64, t11783: f64, t4845: f64, t15745: f64, t3215: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55104 = t11774 * t53405 * t16103;
    let t55122 = t372 * t16170;
    let t55137 = t12116 * t15688;
    let t55141 = t15925 * t11773;
    let t55148 = t11783 * t4845;
    let t55150 = t15745 * t3215;
    (t55104, t55122, t55137, t55141, t55148, t55150)
}

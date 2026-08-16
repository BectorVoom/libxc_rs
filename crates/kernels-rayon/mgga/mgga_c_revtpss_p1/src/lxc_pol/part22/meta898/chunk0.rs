//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3090/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3090(t42865: f64, t72: f64, t3088: f64, t43472: f64, t43401: f64, t11710: f64, t15969: f64, t4892: f64, t1062: f64, t15655: f64, t11643: f64, t15707: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53667 = t42865 * t72;
    let t53668 = t3088 * t53667;
    let t53669 = t43472 * t53668;
    let t53676 = t43401 * t53668;
    let t53690 = t4892 * t11710 * t15969;
    let t53692 = t15655 * t1062;
    let t53710 = t15707 * t11643;
    (t53667, t53668, t53669, t53676, t53690, t53692, t53710)
}

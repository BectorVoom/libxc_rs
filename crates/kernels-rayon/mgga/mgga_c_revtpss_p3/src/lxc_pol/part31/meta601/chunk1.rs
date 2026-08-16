//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2034/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2034(t1904: f64, t25912: f64, t689: f64, t1385: f64, t7910: f64, t14104: f64, t94725: f64, t1358: f64, t2439: f64, t785: f64, t2435: f64, t7925: f64) -> (f64, f64, f64, f64, f64) {
    let t97869 = 0.10975748638225852664e-1_f64 * t689 * t25912 * t1904;
    let t97875 = t1385 * t7910;
    let t97882 = t94725 * t14104;
    let t97894 = t2439 * t785 * t7910 * t1358;
    let t97899 = t7925 * t2435;
    (t97869, t97875, t97882, t97894, t97899)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 999/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk999(t1277: f64, t24514: f64, t1774: f64, t3737: f64, t6702: f64, t1828: f64, t13182: f64, t13100: f64, t24228: f64, t247: f64, t1794: f64, t6628: f64) -> (f64, f64, f64, f64, f64) {
    let t24515 = t1277 * t24514;
    let t24519 = t3737 * t1774 * t6702;
    let t24524 = t6702 * t1828;
    let t24525 = t13182 * t24524;
    let t24535 = t247 * t13100 * t24228;
    let t24543 = t6628 * t1794;
    (t24515, t24519, t24525, t24535, t24543)
}

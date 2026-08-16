//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2033;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta601(t94589: f64, t97814: f64, t2435: f64, t27965: f64, t14090: f64, t26054: f64, t25894: f64, t97676: f64, t97680: f64, t14110: f64, t94901: f64, t10073: f64, t1903: f64, t2029: f64, t25929: f64, t1904: f64, t25912: f64, t689: f64, t1385: f64, t7910: f64, t14104: f64, t94725: f64, t1358: f64, t2439: f64, t785: f64, t7925: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97815, t97823, t97825, t97838, t97843, t97847) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2033(t94589, t97814, t2435, t27965, t14090, t26054, t25894, t97676, t97680, t14110, t94901, t10073, t1903, t2029, t25929);
        let (t97869, t97875, t97882, t97894, t97899) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2034(t1904, t25912, t689, t1385, t7910, t14104, t94725, t1358, t2439, t785, t2435, t7925);
    (t97815, t97823, t97825, t97838, t97843, t97847, t97869, t97875, t97882, t97894, t97899)
}

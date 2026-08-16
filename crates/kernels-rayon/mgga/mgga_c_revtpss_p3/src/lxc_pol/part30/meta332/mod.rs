//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1338;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta332(t2645: f64, t2723: f64, t10115: f64, t253: f64, t233: f64, t2760: f64, t869: f64, t689: f64, t2777: f64, t2789: f64, t2439: f64, t2435: f64, t2790: f64, t2778: f64, t9303: f64, t871: f64, t9292: f64, t72: f64, t686: f64, t874: f64, t251: f64, t9646: f64, t22: f64, t780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10943, t10948, t10961, t10964, t10966) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1338(t2645, t2723, t10115, t253, t233, t2760, t869, t689, t2777, t2789, t2439, t2435, t2790);
        let (t10969, t10971, t10974, t10981, t10982) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1339(t2778, t9303, t871, t9292, t2760, t72, t686, t874, t251, t9646, t22, t780);
    (t10943, t10948, t10961, t10964, t10966, t10969, t10971, t10974, t10981, t10982)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1375;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1376;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta356(t10139: f64, t14220: f64, t13926: f64, t543: f64, t4100: f64, t2782: f64, t10014: f64, t5741: f64, t13790: f64, t1398: f64, t10022: f64, t1892: f64, t4086: f64, t786: f64, t4104: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14221, t14224) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1375(t10139, t14220, t13926, t543);
        let (t14227, t14229, t14230, t14233, t14239, t14241) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1376(t14224, t4100, t2782, t10014, t5741, t13790, t1398, t10022, t1892, t4086, t786, t4104);
    (t14221, t14224, t14227, t14229, t14230, t14233, t14239, t14241)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2053;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta592(t665: f64, t94975: f64, t2339: f64, t624: f64, t2340: f64, t2366: f64, t25823: f64, t10208: f64, t68: f64, t25081: f64, t7234: f64, t116: f64, t25832: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t94976, t94978, t94979, t94981, t94982, t95088, t95137) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2053(t665, t94975, t2339, t624, t2340, t2366, t25823, t10208, t68, t25081, t7234, t116, t25832);
    (t94976, t94978, t94979, t94981, t94982, t95088, t95137)
}

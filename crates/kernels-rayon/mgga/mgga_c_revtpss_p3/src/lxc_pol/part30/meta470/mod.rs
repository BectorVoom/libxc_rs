//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta470(t25331: f64, t7064: f64, t1949: f64, t785: f64, t780: f64, t2439: f64, t212: f64, t7048: f64, t689: f64, t231: f64, t836: f64, t7076: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25333, t25334, t25335, t25337, t25338, t25339, t25340, t25344) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1778(t25331, t7064, t1949, t785, t780, t2439, t212, t7048, t689, t231, t836, t7076);
    (t25333, t25334, t25335, t25337, t25338, t25339, t25340, t25344)
}

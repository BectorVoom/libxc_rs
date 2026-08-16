//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta483(t25331: f64, t7064: f64, t1949: f64, t785: f64, t780: f64, t2439: f64, t212: f64, t7048: f64, t689: f64, t7014: f64, t887: f64, t7049: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25333, t25334, t25335, t25337, t25338, t25339, t25340, t25352, t25353, t25355) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1763(t25331, t7064, t1949, t785, t780, t2439, t212, t7048, t689, t7014, t887, t7049, t786);
    (t25333, t25334, t25335, t25337, t25338, t25339, t25340, t25352, t25353, t25355)
}

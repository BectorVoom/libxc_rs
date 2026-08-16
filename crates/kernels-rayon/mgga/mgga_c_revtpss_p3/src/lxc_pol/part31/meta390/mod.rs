//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1429;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1430;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta390(t5048: f64, t689: f64, t5053: f64, t5057: f64) -> (f64, f64, f64, f64) {
        let t16708 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1429(t5048, t689);
        let t16710 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1430(t5053, t689);
        let (t16711, t16712) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1431(t16710, t5057, t689);
    (t16708, t16710, t16711, t16712)
}

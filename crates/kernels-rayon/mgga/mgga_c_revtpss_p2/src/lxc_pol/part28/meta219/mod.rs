//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1039;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1040;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta219(t3317: f64, t4891: f64, t1043: f64, t357: f64, t4893: f64, t3117: f64, t1651: f64, t1045: f64, t999: f64, t4781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4899, t4900, t4901, t4902) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1039(t3317, t4891, t1043, t357, t4893, t3117);
        let (t4905, t4906, t4907) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1040(t1043, t1651, t1045, t3117);
        let (t4910, t4911, t4912) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1041(t357, t999, t4781, t3117);
    (t4899, t4900, t4901, t4902, t4905, t4906, t4907, t4910, t4911, t4912)
}

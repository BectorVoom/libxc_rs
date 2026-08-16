//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1819;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1820;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta483(t25901: f64, t25904: f64, t1955: f64, t4066: f64, t212: f64, t7274: f64, t1358: f64, t689: f64, t2022: f64, t785: f64, t2439: f64, t1032: f64, t1419: f64, t545: f64, t9656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25905, t25909, t25912, t25913, t25914, t25916, t25917, t25919, t25920) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1819(t25901, t25904, t1955, t4066, t212, t7274, t1358, t689, t2022, t785, t2439, t1032, t1419);
        let t25921 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1820(t1955, t25920);
        let t25924 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1821(t545, t9656);
    (t25905, t25909, t25912, t25913, t25914, t25916, t25917, t25919, t25920, t25921, t25924)
}

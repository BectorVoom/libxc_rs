//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1854;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1855;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1856;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1857;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1858;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta489(t25875: f64, t25877: f64, t122: f64, t2022: f64, t72: f64, t3916: f64, t4131: f64, t7296: f64, t1444: f64, t7274: f64, t2435: f64, t7243: f64, t555: f64, t786: f64, t1385: f64, t2028: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t25878 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1854(t25875, t25877);
        let t25880 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1855(t122, t2022, t72);
        let t25881 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1856(t25880, t3916);
        let (t25882, t25884, t25885, t25889, t25893, t25894) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1857(t25878, t25881, t2022, t4131, t7296, t1444, t7274, t2435, t7243, t555, t786);
        let t25895 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1858(t25877, t25894);
        let (t25896, t25898) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1859(t25881, t25895, t1385, t2028);
    (t25878, t25880, t25881, t25882, t25884, t25885, t25889, t25893, t25894, t25895, t25896, t25898)
}

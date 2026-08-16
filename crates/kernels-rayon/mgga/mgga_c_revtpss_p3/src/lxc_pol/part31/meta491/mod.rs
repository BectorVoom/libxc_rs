//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1795;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1796;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1797;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1798;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta491(t25875: f64, t25877: f64, t122: f64, t2022: f64, t72: f64, t3916: f64, t2435: f64, t7243: f64, t555: f64, t786: f64, t1385: f64, t2028: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t25878 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1795(t25875, t25877);
        let (t25880, t25881, t25882, t25893, t25894) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1796(t122, t2022, t72, t3916, t25878, t2435, t7243, t555, t786);
        let t25895 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1797(t25877, t25894);
        let (t25896, t25898) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1798(t25881, t25895, t1385, t2028);
        let t25899 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1799(t25875, t25898);
    (t25878, t25880, t25881, t25882, t25893, t25894, t25895, t25896, t25898, t25899)
}

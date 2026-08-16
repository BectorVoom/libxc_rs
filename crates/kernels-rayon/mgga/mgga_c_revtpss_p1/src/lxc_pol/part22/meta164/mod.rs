//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1099;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1100;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta164(t1359: f64, t2435: f64, t555: f64, t785: f64, t1358: f64, t2439: f64, t1419: f64, t212: f64, t689: f64, t1357: f64, t1445: f64, t2453: f64, t556: f64, t136: f64, t561: f64, t2457: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3894, t3895) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1099(t1359, t2435, t555, t785);
        let (t3896, t3898, t3899) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1100(t1358, t3895, t2439, t1419, t212);
        let (t3900, t3901, t3903, t3904, t3906, t3907, t3908) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1101(t1358, t3899, t689, t1357, t1445, t2453, t556, t136, t561, t2457);
    (t3894, t3895, t3896, t3898, t3899, t3900, t3901, t3903, t3904, t3906, t3907, t3908)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1776;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1777;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1778;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta486<F: Float>(t25504: F, t3141: F, t3148: F, t7120: F, t3123: F, t7121: F, t365: F, sigma0: F, t3089: F, t1087: F, t1024: F, t7131: F) -> (F, F, F, F, F, F, F, F) {
        let (t25505, t25508, t25509, t25512, t25515) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1776::<F>(t25504, t3141, t3148, t7120, t3123, t7121, t365, sigma0);
        let t25516 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1777::<F>(t25515, t3089);
        let t25517 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1778::<F>(t1087, t25516);
        let t25522 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1779::<F>(t1024, t7131);
    (t25505, t25508, t25509, t25512, t25515, t25516, t25517, t25522)
}

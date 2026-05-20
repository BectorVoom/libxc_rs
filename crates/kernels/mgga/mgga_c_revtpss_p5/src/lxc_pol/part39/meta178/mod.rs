//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk772;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk773;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk774;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta178<F: Float>(t1444: F, t676: F, t123: F, t3915: F, t1363: F, t2470: F, t1362: F, t1398: F, t543: F, t1390: F, t828: F, t1386: F, t820: F, t843: F, t1401: F, t241: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3916, t3917, t3918, t3920, t3922, t3923) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk772::<F>(t1444, t676, t123, t3915, t1363, t2470, t1362, t1398);
        let t3924 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk773::<F>(t3923, t543);
        let (t3926, t3930) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk774::<F>(t1390, t3924, t828, t1386, t820, t843);
        let (t3931, t3934) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk775::<F>(t1401, t3930, t1386, t241, t820);
    (t3916, t3917, t3918, t3920, t3922, t3923, t3924, t3926, t3930, t3931, t3934)
}

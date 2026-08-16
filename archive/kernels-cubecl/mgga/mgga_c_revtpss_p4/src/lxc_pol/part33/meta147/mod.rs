//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta147 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk769;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk770;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta147<F: Float>(t3906: F, t3908: F, t1420: F, t786: F, t1364: F, t1426: F, t556: F, t1444: F, t676: F, t123: F, t1363: F, t2470: F, t1362: F, t1386: F, t820: F, t843: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3910, t3911, t3912, t3914, t3915) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk769::<F>(t3906, t3908, t1420, t786, t1364, t1426, t556);
        let (t3916, t3917, t3918, t3920) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk770::<F>(t1444, t676, t123, t3915, t1363, t2470);
        let (t3922, t3930) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk771::<F>(t1362, t3920, t1386, t820, t843);
    (t3910, t3911, t3912, t3914, t3915, t3916, t3917, t3918, t3920, t3922, t3930)
}

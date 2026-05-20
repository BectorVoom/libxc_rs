//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1845;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1846;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta351<F: Float>(t1016: F, t697: F, t1011: F, t1010: F, t2270: F, t3241: F, t3244: F, t1058: F, t3197: F, t11132: F, t3163: F, t3172: F, t3161: F, t126: F, t373: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11880, t11881, t11883) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1845::<F>(t1016, t697, t1011, t1010, t2270);
        let (t11886, t11888, t11890, t11916, t11917, t11921) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1846::<F>(t3241, t3244, t1058, t3197, t11132, t3163, t3172, t3161, t126, t373);
        let t11922 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1847::<F>(t11921, t828);
    (t11880, t11881, t11883, t11886, t11888, t11890, t11916, t11917, t11921, t11922)
}

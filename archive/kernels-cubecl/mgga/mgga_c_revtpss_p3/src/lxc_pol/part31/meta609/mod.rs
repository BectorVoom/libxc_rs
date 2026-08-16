//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2049;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2050;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta609<F: Float>(t26004: F, t5690: F, t13951: F, t2018: F, t807: F, t25240: F, t3964: F, t5617: F, t27857: F, t689: F, t25904: F, t786: F, t97961: F, t7286: F, t2439: F, t7925: F, t94391: F, t94383: F, t25878: F, t98028: F, t94771: F, t97814: F, t1903: F, t25931: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98270, t98282, t98285, t98303, t98305, t98308) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2049::<F>(t26004, t5690, t13951, t2018, t807, t25240, t3964, t5617, t27857, t689, t25904, t786, t97961);
        let (t98310, t98312, t98314, t98333, t98338, t98340) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2050::<F>(t7286, t98308, t2439, t7925, t94391, t94383, t25878, t98028, t94771, t97814, t1903, t25931);
    (t98270, t98282, t98285, t98303, t98305, t98310, t98312, t98314, t98333, t98338, t98340)
}

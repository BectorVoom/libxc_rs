//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta814 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2920;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta814<F: Float>(t2453: F, t3908: F, t4067: F, t10115: F, t1421: F, t10168: F, t3920: F, t10174: F, t9676: F, t123: F, t2434: F, t3915: F, t4131: F, t10175: F, t9686: F, t1420: F, t4075: F, t786: F, t2439: F, t3895: F, t4132: F, t1359: F, t39501: F, t555: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47510, t47512, t47516, t47520, t47521, t47525) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2920::<F>(t2453, t3908, t4067, t10115, t1421, t10168, t3920, t10174, t9676, t123, t2434, t3915, t4131);
        let (t47527, t47530, t47534, t47561, t47567) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2921::<F>(t10175, t9686, t1420, t4075, t786, t2439, t3895, t4132, t1359, t39501, t10115, t555);
    (t47510, t47512, t47516, t47520, t47521, t47525, t47527, t47530, t47534, t47561, t47567)
}

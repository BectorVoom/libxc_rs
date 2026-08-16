//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1655;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1656;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1657;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta442<F: Float>(t25227: F, t2664: F, t2661: F, t2670: F, t7033: F, t2482: F, t27: F, t7043: F, t2677: F, t1941: F, t243: F, t2732: F, t2712: F, t64: F, t2710: F, t826: F, t7036: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25228, t25229, t25230, t25231, t25234) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1655::<F>(t25227, t2664, t2661, t2670, t7033, t2482, t27, t7043);
        let (t25235, t25236, t25238, t25240) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1656::<F>(t25234, t2677, t1941, t243, t2732, t2712, t64);
        let (t25242, t25245) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1657::<F>(t25240, t2710, t826, t2482, t27, t7036);
    (t25228, t25229, t25230, t25231, t25234, t25235, t25236, t25238, t25240, t25242, t25245)
}

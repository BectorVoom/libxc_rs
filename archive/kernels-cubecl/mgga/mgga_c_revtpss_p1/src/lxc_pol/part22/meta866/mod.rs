//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta866 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3021;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3022;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta866<F: Float>(t14792: F, t50768: F, t50769: F, t14688: F, t40731: F, t10777: F, t14671: F, t14686: F, t2754: F, t14749: F, t221: F, t10703: F, t2674: F, t4398: F, t9323: F, t4302: F, t9586: F, t10612: F, t4311: F, t14330: F, t14369: F, t2251: F, t14440: F, t2398: F, t2258: F, t4401: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t50771, t50773, t50784, t50791) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3021::<F>(t14792, t50768, t50769, t14688, t40731, t10777, t14671, t14686, t2754, t14749, t221, t10703, t2674);
        let (t50852, t50856, t50865, t50868, t50873, t50878) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3022::<F>(t4398, t9323, t4302, t9586, t10612, t4311, t14330, t14369, t2251, t14440, t2398, t2258, t4401);
    (t50771, t50773, t50784, t50791, t50852, t50856, t50865, t50868, t50873, t50878)
}

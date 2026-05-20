//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta736 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2510;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2511;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta736<F: Float>(t10886: F, t14833: F, t808: F, t241: F, t40322: F, t820: F, t2659: F, t2783: F, t816: F, t853: F, t14688: F, t40731: F, t4398: F, t9323: F, t4302: F, t9586: F, t10612: F, t4311: F, t14440: F, t2398: F, t14322: F, t2626: F, t9425: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50707, t50757, t50768, t50769, t50773) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2510::<F>(t10886, t14833, t808, t241, t40322, t820, t2659, t2783, t816, t853, t14688, t40731);
        let (t50774, t50852, t50856, t50866, t50874, t50884, t50888) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2511::<F>(t50773, t4398, t9323, t4302, t9586, t10612, t4311, t14440, t2398, t14322, t2626, t9425);
    (t50707, t50757, t50768, t50769, t50774, t50852, t50856, t50866, t50874, t50884, t50888)
}

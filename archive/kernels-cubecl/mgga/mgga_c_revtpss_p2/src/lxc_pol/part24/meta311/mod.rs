//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta311<F: Float>(t3930: F, t6846: F, t221: F, t4019: F, t6862: F, t10001: F, t6800: F, t72: F, t757: F, t1317: F, t6801: F, t1320: F) -> (F, F, F, F, F, F, F) {
        let (t22179, t22182, t22183, t22185, t22186, t22188, t22191) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1098::<F>(t3930, t6846, t221, t4019, t6862, t10001, t6800, t72, t757, t1317, t6801, t1320);
    (t22179, t22182, t22183, t22185, t22186, t22188, t22191)
}

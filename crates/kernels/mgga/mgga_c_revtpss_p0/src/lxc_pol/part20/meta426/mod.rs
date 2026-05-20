//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1601;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1602;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta426<F: Float>(t3431: F, t408: F, t3434: F, t44018: F, t3427: F, t3433: F, t3435: F, t1126: F, t12247: F, t12249: F, t12227: F, t12243: F, t12364: F, t1150: F, t12248: F, t3384: F, t1188: F, t1196: F, t3495: F, t43966: F, t3798: F, t3800: F, t3140: F, t3552: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t44096, t44097, t44100, t44103, t44106, t44108) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1601::<F>(t3431, t408, t3434, t44018, t3427, t3433, t3435, t1126, t12247, t12249, t12227, t12243, t12364);
        let (t44111, t44114, t44122, t44123, t44126, t44169) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1602::<F>(t1150, t12248, t44018, t3384, t44097, t1188, t1196, t3495, t43966, t3798, t3800, t3140, t3552);
    (t44096, t44100, t44103, t44106, t44108, t44111, t44114, t44122, t44123, t44126, t44169)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1841;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1842;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1843;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta350<F: Float>(t1025: F, t11817: F, t271: F, t2857: F, t283: F, t66: F, t3298: F, t994: F, t4891: F, t3154: F, t999: F, t1086: F, t3046: F, t3090: F, t3316: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11818, t11821, t11852) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1841::<F>(t1025, t11817, t271, t2857, t283);
        let (t11853, t11858, t11859) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1842::<F>(t11852, t66, t3298, t994, t4891);
        let (t11860, t11865, t11866) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1843::<F>(t3154, t999, t1086, t3046, t3090);
        let (t11874, t11875) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1844::<F>(t3316, t994, t4891);
    (t11818, t11821, t11852, t11853, t11858, t11859, t11860, t11865, t11866, t11874, t11875)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1045;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1046;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1047;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1048;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta164<F: Float>(t187: F, t3850: F, t2608: F, t520: F, t512: F, t189: F, t19: F, t27: F, t521: F, t14: F, t22: F, t583: F, t588: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3852, t3853) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1045::<F>(t187, t3850, t2608, t520);
        let (t3854, t3855) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1046::<F>(t3853, t512, t189, t3850);
        let (t3856, t3857) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1047::<F>(t3855, t512, t19, t27);
        let (t3859, t3860) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1048::<F>(t3857, t521, t14, t22);
        let (t3862, t3863) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1049::<F>(t3860, t521, t583, t588);
    (t3852, t3853, t3854, t3855, t3856, t3857, t3859, t3860, t3862, t3863)
}

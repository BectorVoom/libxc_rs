//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta109 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk626;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk627;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk628;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta109<F: Float>(t3857: F, t521: F, t14: F, t22: F, t583: F, t588: F, t1320: F, t1333: F, t123: F, t520: F) -> (F, F, F, F, F, F, F) {
        let (t3859, t3860) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk626::<F>(t3857, t521, t14, t22);
        let (t3862, t3863) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk627::<F>(t3860, t521, t583, t588);
        let (t3865, t3867, t3869) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk628::<F>(t3863, t521, t1320, t1333, t123, t520);
    (t3859, t3860, t3862, t3863, t3865, t3867, t3869)
}

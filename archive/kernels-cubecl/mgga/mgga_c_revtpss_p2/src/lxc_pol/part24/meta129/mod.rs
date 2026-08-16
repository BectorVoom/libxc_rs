//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta129 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk681;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk682;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk683;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk684;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta129<F: Float>(t354: F, t357: F, t3298: F, t378: F, t342: F, t3154: F, t3302: F, t3316: F, t1678: F, t359: F, t198: F, t336: F, t1716: F, t689: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4975, t4980) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk681::<F>(t354, t357, t3298, t378);
        let (t4981, t4982, t4995) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk682::<F>(t342, t4980, t3154, t3302, t3316, t378);
        let (t4996, t5004) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk683::<F>(t342, t4995, t1678, t359);
        let t5023 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk684::<F>(t198, t336);
        let t5044 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk685::<F>(t1716, t689);
    (t4975, t4980, t4981, t4982, t4995, t4996, t5004, t5023, t5044)
}

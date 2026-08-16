//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta175 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1047;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1048;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1049;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1050;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1051;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1052;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1053;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta175<F: Float>(t1082: F, t4757: F, t1089: F, t4905: F, t1651: F, t3291: F, t4772: F, t354: F, t357: F, t999: F, t4781: F, t3298: F, t378: F, t342: F, t3154: F, t3302: F, t1043: F, t4893: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4961, t4964, t4967, t4970, t4975, t4976) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1047::<F>(t1082, t4757, t1089, t4905, t1651, t3291, t4772, t354, t357, t999);
        let t4977 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1048::<F>(t4781, t4976);
        let t4980 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1049::<F>(t3298, t378);
        let t4981 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1050::<F>(t342, t4980);
        let t4982 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1051::<F>(t3154, t3302);
        let t4983 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1052::<F>(t1043, t4982);
        let t4984 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1053::<F>(t4893, t4983);
    (t4961, t4964, t4967, t4970, t4975, t4976, t4977, t4980, t4981, t4982, t4983, t4984)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk793;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk794;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta158<F: Float>(t225: F, t385: F, t6343: F, t1695: F, t3269: F, t1082: F, t6244: F, t1089: F, t6271: F, t1651: F, t5004: F, t6258: F, t378: F, t6305: F, t3304: F, t1668: F, t1678: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6345, t6350) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk793::<F>(t225, t385, t6343, t1695);
        let t6351 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk794::<F>(t3269, t6350);
        let (t6362, t6365, t6368, t6371, t6374, t6375, t6379) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk795::<F>(t1082, t6244, t1089, t6271, t1651, t5004, t6258, t378, t6305, t3304, t1668, t1678);
    (t6345, t6350, t6351, t6362, t6365, t6368, t6371, t6374, t6375, t6379)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta253 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1565;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1566;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1567;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1568;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta253<F: Float>(t373: F, t6244: F, t371: F, t372: F, t1041: F, t1063: F, t1671: F, t1675: F, t3150: F, t3161: F, t3203: F, t3205: F, t375: F, t4834: F, t4846: F, t4879: F, t4925: F, t6302: F, t6308: F, t6312: F, t6318: F, t6323: F, t6327: F, t6331: F, t6298: F, t225: F, t385: F, t1695: F, t3269: F) -> (F, F, F, F, F, F) {
        let (t6337, t6339) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1565::<F>(t373, t6244, t371, t372);
        let t6342 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1566::<F>(t1041, t1063, t1671, t1675, t3150, t3161, t3203, t3205, t375, t4834, t4846, t4879, t4925, t6302, t6308, t6312, t6318, t6323, t6327, t6331, t6339);
        let t6343 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1567::<F>(t6298, t6342);
        let (t6345, t6350) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1568::<F>(t225, t385, t6343, t1695);
        let t6351 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1569::<F>(t3269, t6350);
    (t6337, t6339, t6343, t6345, t6350, t6351)
}

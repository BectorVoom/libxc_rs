//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk790;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk791;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta157<F: Float>(t373: F, t6305: F, t3155: F, t1042: F, t3162: F, t225: F, t6235: F, t366: F, t1066: F, t6100: F, t247: F, t3182: F, t6092: F, t6096: F, t6244: F, t371: F, t372: F, t1041: F, t1063: F, t1671: F, t1675: F, t3150: F, t3161: F, t3203: F, t3205: F, t375: F, t4834: F, t4846: F, t4879: F, t4925: F, t6302: F, t6298: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6307, t6308, t6311, t6312, t6317, t6318, t6323, t6326) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk790::<F>(t373, t6305, t3155, t1042, t3162, t225, t6235, t366, t1066, t6100, t247, t3182, t6092);
        let (t6327, t6331, t6337, t6339, t6342) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk791::<F>(t247, t6326, t1066, t6096, t373, t6244, t371, t372, t1041, t1063, t1671, t1675, t3150, t3161, t3203, t3205, t375, t4834, t4846, t4879, t4925, t6302, t6308, t6312, t6318, t6323);
        let t6343 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk792::<F>(t6298, t6342);
    (t6307, t6308, t6311, t6312, t6317, t6318, t6323, t6327, t6331, t6337, t6339, t6343)
}

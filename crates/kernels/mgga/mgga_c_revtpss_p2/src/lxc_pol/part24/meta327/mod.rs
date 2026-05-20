//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1136;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1137;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta327<F: Float>(t22213: F, t13666: F, t13668: F, t13670: F, t13887: F, t9524: F, t9542: F, t9588: F, t9598: F, t9854: F, t9857: F, t9865: F, t9868: F, t225: F, t22917: F, t22923: F, t22927: F, t22813: F, t9880: F, t5651: F, t6816: F, t1394: F, t22809: F, t1877: F, t1879: F, t539: F, t541: F, t5650: F, t6832: F, t6837: F, t6840: F, t543: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22928, t22929, t22930, t22931, t22932, t22933) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1136::<F>(t22213, t13666, t13668, t13670, t13887, t9524, t9542, t9588, t9598, t9854, t9857, t9865, t9868);
        let (t22936, t22944, t22947, t22950, t22953) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1137::<F>(t225, t22917, t22923, t22927, t22933, t22813, t9880, t5651, t6816, t1394, t22809, t1877, t1879, t539, t541, t5650, t6832, t6837, t6840);
        let t22954 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1138::<F>(t22953, t543);
    (t22928, t22929, t22930, t22931, t22932, t22936, t22944, t22947, t22950, t22953, t22954)
}

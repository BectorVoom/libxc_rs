//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2052;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2053;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta617<F: Float>(t1889: F, t94545: F, t13846: F, t13877: F, t7021: F, t5665: F, t94497: F, t14036: F, t25997: F, t13941: F, t94423: F, t14005: F, t5706: F, t94429: F, t1941: F, t9817: F, t5651: F, t7028: F, t9736: F, t13985: F, t13878: F, t25972: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98165, t98169, t98174, t98181, t98186, t98187) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2052::<F>(t1889, t94545, t13846, t13877, t7021, t5665, t94497, t14036, t25997, t13941, t94423, t14005);
        let (t98188, t98194, t98196, t98200, t98203, t98206) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2053::<F>(t98187, t5706, t94429, t1941, t9817, t5651, t7028, t9736, t13985, t94423, t13878, t25972);
    (t98165, t98169, t98174, t98181, t98186, t98188, t98194, t98196, t98200, t98203, t98206)
}

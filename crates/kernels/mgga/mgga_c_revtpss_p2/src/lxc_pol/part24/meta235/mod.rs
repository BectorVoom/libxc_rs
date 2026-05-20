//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk994;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk995;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta235<F: Float>(t1408: F, t240: F, t5610: F, t9775: F, t1889: F, t9779: F, t828: F, t9954: F, t3935: F, t1882: F, t4003: F, t1873: F, t9741: F, t5651: F, t808: F, t9736: F, t241: F, t820: F, t9991: F, t2482: F, t4000: F, t814: F, t136: F, t550: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13767, t13779, t13781, t13783, t13789, t13790) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk994::<F>(t1408, t240, t5610, t9775, t1889, t9779, t828, t9954, t3935, t1882, t4003);
        let (t13798, t13800, t13801, t13804, t13845, t13846) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk995::<F>(t1873, t9741, t5651, t808, t9736, t241, t820, t9991, t2482, t4000, t814, t136, t550);
    (t13767, t13779, t13781, t13783, t13789, t13790, t13798, t13800, t13801, t13804, t13845, t13846)
}

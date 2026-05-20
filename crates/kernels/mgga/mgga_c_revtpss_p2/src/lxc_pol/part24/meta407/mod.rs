//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1347;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1348;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta407<F: Float>(t235: F, t4503: F, t2453: F, t123: F, t125: F, t2452: F, t40633: F, t810: F, t10759: F, t2735: F, t10293: F, t240: F, t243: F, t813: F, t816: F, t798: F, t9726: F, t10899: F, t794: F, t159: F, t216: F, t2475: F, t251: F, t40321: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40799, t40810, t40834, t40846) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1347::<F>(t235, t4503, t2453, t123, t125, t2452, t40633, t810, t10759, t2735, t10293, t240);
        let (t40850, t40861, t40864, t40868, t40902) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1348::<F>(t243, t40846, t813, t816, t798, t9726, t10899, t794, t159, t216, t2475, t251, t40321);
    (t40799, t40810, t40834, t40846, t40850, t40861, t40864, t40868, t40902)
}

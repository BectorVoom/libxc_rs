//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta266<F: Float>(t1420: F, t2453: F, t3908: F, t1426: F, t786: F, t64: F, t843: F, t112: F, t2289: F, t666: F, t654: F, t98: F, t99: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10165, t10166, t10174, t10175, t10199, t10201, t10202, t10207, t10208, t10226) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1476::<F>(t1420, t2453, t3908, t1426, t786, t64, t843, t112, t2289, t666, t654, t98, t99);
    (t10165, t10166, t10174, t10175, t10199, t10201, t10202, t10207, t10208, t10226)
}

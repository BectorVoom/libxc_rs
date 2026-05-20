//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta165 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk882;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk883;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk884;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk885;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta165<F: Float>(t2580: F, t680: F, t130: F, t146: F, t2583: F, t9275: F, t2514: F, t2596: F, t746: F, t1340: F, t2491: F, t2495: F, t744: F, t215: F, t681: F, t268: F, t702: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9310, t9311, t9313, t9314, t9316) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk882::<F>(t2580, t680, t130, t146, t2583, t9275);
        let t9318 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk883::<F>(t2514, t2596, t746);
        let (t9320, t9321, t9323) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk884::<F>(t1340, t9318, t2491, t2514, t2495, t744);
        let (t9325, t9326, t9329) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk885::<F>(t1340, t9323, t215, t681, t268, t702);
    (t9310, t9311, t9313, t9314, t9316, t9318, t9320, t9321, t9323, t9325, t9326, t9329)
}

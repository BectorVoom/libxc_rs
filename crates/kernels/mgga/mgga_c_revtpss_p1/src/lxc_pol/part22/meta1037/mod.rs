//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1037 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3626;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1037<F: Float>(t20382: F, t3520: F, t1196: F, t5206: F, t12500: F, t20895: F, t5205: F, t58000: F, t1757: F, t58708: F, t68605: F, t16662: F, t57818: F, t20394: F, t3531: F, t20896: F, t12571: F, t6556: F, t20890: F, t43977: F, t68631: F, t68633: F, t68636: F, t68640: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t68683, t68686, t68689, t68692, t68694) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3626::<F>(t20382, t3520, t1196, t5206, t12500, t20895, t5205, t58000, t1757, t58708, t68605, t16662, t57818);
        let (t68696, t68698, t68700, t68703, t68704) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3627::<F>(t20394, t3531, t20896, t12571, t6556, t1196, t20890, t43977, t68631, t68633, t68636, t68640, t68683, t68686, t68689, t68692, t68694);
    (t68683, t68686, t68689, t68692, t68694, t68696, t68698, t68700, t68703, t68704)
}

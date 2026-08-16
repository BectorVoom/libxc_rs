//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1474;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta483<F: Float>(t3671: F, t371: F, t6609: F, t676: F, t480: F, t69637: F, t17303: F, t5323: F, t5327: F, t1284: F, t20849: F, t3624: F, t3625: F, t44250: F, t6639: F, t21439: F, t11249: F, t6622: F, t3682: F, t6667: F, t474: F, t6593: F, t3089: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t70511, t70578, t70583, t70758, t70800) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1474::<F>(t3671, t371, t6609, t676, t480, t69637, t17303, t5323, t5327, t1284, t20849, t3624);
        let (t70809, t70819, t70890, t70942, t70994) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1475::<F>(t3625, t44250, t6639, t21439, t3624, t11249, t6622, t3682, t6667, t474, t6593, t3089);
    (t70511, t70578, t70583, t70758, t70800, t70809, t70819, t70890, t70942, t70994)
}

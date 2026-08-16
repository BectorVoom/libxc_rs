//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2439;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2440;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta581<F: Float>(t18534: F, t18553: F, t18568: F, t18583: F, t225: F, t1553: F, t73: F, t2475: F, t5966: F, t775: F, t4343: F, t4416: F, t5962: F, t853: F, t18392: F, t832: F, t1555: F, t227: F, t229: F, t4409: F, t4415: F, t4417: F, t4420: F, t6006: F, t6010: F, t6013: F, t830: F, t833: F, t231: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18586, t18592, t18599, t18600, t18603) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2439::<F>(t18534, t18553, t18568, t18583, t225, t1553, t73, t2475, t5966, t775, t4343, t4416);
        let (t18608, t18609, t18612, t18615) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2440::<F>(t5962, t853, t775, t18392, t832, t1553, t1555, t18586, t18592, t18600, t18603, t227, t229, t4409, t4415, t4417, t4420, t6006, t6010, t6013, t830, t833);
        let t18616 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2441::<F>(t18615, t231);
    (t18586, t18592, t18599, t18600, t18603, t18608, t18609, t18612, t18615, t18616)
}

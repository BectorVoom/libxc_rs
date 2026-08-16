//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1186;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1187;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta184<F: Float>(t125: F, t1558: F, t2723: F, t836: F, t4364: F, t1544: F, t854: F, t236: F, t807: F, t2498: F, t2518: F, t2522: F, t2526: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t4300: F, t4301: F, t4304: F) -> (F, F, F, F, F, F, F) {
        let t4365 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1186::<F>(t125, t1558);
        let t4366 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1187::<F>(t2723, t836);
        let (t4368, t4371, t4372, t4373, t4376) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1188::<F>(t4365, t4366, t4364, t1544, t854, t236, t807, t2498, t2518, t2522, t2526, t2562, t2569, t2579, t2587, t2610, t4300, t4301, t4304);
    (t4365, t4366, t4368, t4371, t4372, t4373, t4376)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta152 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk942;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk943;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk944;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta152<F: Float>(t2662: F, t4353: F, t2661: F, t1565: F, t2652: F, t1561: F, t2741: F, t241: F, t2719: F, t820: F, t243: F, t72: F, t245: F, t125: F, t1558: F, t2723: F, t836: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4354, t4355, t4357, t4359, t4362) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk942::<F>(t2662, t4353, t2661, t1565, t2652, t1561, t2741, t241, t2719, t820);
        let (t4363, t4364) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk943::<F>(t243, t72, t245);
        let t4365 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk944::<F>(t125, t1558);
        let t4366 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk945::<F>(t2723, t836);
    (t4354, t4355, t4357, t4359, t4362, t4363, t4364, t4365, t4366)
}

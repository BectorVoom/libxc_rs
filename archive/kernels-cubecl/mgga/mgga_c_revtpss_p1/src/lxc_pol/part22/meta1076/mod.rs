//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1076 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3856;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1076<F: Float>(t47093: F, t39989: F, t47084: F, t47086: F, t47088: F, t47092: F, t47096: F, t74114: F, t74115: F, t74116: F, t74117: F, t74119: F, t74120: F, t74121: F, t74122: F, t74123: F, t74124: F, t74125: F, t47099: F, t22212: F, t2626: F, t1320: F, t22195: F, t47101: F, t48313: F, t47110: F, t47113: F, t47119: F, t47125: F, t40067: F, t40072: F, t47098: F, t47109: F, t47116: F, t47118: F, t47122: F, t47124: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t74126, t74127) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3856::<F>(t47093, t39989, t47084, t47086, t47088, t47092, t47096, t74114, t74115, t74116, t74117, t74119, t74120, t74121, t74122, t74123, t74124, t74125);
        let (t74129, t74131, t74133, t74134, t74135, t74136, t74137, t74138, t74139, t74140) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3857::<F>(t47099, t22212, t2626, t1320, t22195, t47101, t48313, t47110, t47113, t47119, t47125, t40067, t40072, t47098, t47109, t47116, t47118, t47122, t47124);
    (t74126, t74127, t74129, t74131, t74133, t74134, t74135, t74136, t74137, t74138, t74139, t74140)
}

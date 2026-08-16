//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta737 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2512;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2513;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta737<F: Float>(t10565: F, t1532: F, t4398: F, t9419: F, t14362: F, t9572: F, t37: F, t4391: F, t14728: F, t9775: F, t1549: F, t40861: F, t14779: F, t40721: F, t221: F, t40724: F, t14495: F, t40834: F, t826: F, t241: F, t820: F, t849: F, t10886: F, t14652: F, t808: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50892, t50893, t50901, t50903, t50939, t50941) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2512::<F>(t10565, t1532, t4398, t9419, t14362, t9572, t37, t4391, t14728, t9775, t1549, t40861);
        let (t50943, t50945, t50955, t50957, t50977) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2513::<F>(t14779, t40721, t221, t40724, t14495, t40834, t826, t241, t820, t849, t10886, t14652, t808);
    (t50892, t50893, t50901, t50903, t50939, t50941, t50943, t50945, t50955, t50957, t50977)
}

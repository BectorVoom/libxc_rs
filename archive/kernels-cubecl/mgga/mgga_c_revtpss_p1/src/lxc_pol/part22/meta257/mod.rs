//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1586;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta257<F: Float>(t1132: F, t6449: F, t3407: F, t6442: F, t1139: F, t3417: F, t6421: F, t141: F, t1145: F, t6425: F, t6429: F, t3402: F, t3414: F, t5044: F, t5093: F, t6423: F, t6427: F, t6431: F, t6443: F, t1150: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6450, t6456, t6458, t6461, t6462, t6464, t6465, t6467, t6468, t6470) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1586::<F>(t1132, t6449, t3407, t6442, t1139, t3417, t6421, t141, t1145, t6425, t6429, t3402, t3414, t5044, t5093, t6423, t6427, t6431, t6443);
        let t6471 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1587::<F>(t1150, t6470);
    (t6450, t6456, t6458, t6461, t6462, t6464, t6465, t6467, t6468, t6470, t6471)
}

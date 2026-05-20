//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta180 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1173;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1174;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1175;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta180<F: Float>(t4279: F, t661: F, t108: F, t2: F, t580: F, t105: F, t1505: F, t1507: F, t4270: F, t4274: F, t656: F, t662: F, t97: F, t114: F, t655: F, t2335: F, t2336: F, t4261: F, t4264: F, t69: F, t508: F) -> (F, F, F, F, F, F, F) {
        let (t4280, t4283, t4284, t4287) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1173::<F>(t4279, t661, t108, t2, t580, t105, t1505, t1507, t4270, t4274, t656, t662, t97);
        let (t4288, t4292) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1174::<F>(t114, t4287, t655, t2335, t2336, t4261, t4264, t69);
        let t4293 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1175::<F>(t4292, t508);
    (t4280, t4283, t4284, t4287, t4288, t4292, t4293)
}

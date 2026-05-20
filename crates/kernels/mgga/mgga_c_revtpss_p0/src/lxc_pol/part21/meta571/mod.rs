//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2274;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2275;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta571<F: Float>(t17730: F, t5051: F, t3626: F, t3566: F, t489: F, t17728: F, t1121: F, t1774: F, t3584: F, t471: F, t5351: F, t3720: F, t13142: F, t17708: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t17731, t17732, t17736) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2274::<F>(t17730, t5051, t3626, t3566, t489, t17728);
        let (t17737, t17738, t17739, t17742, t17743, t17744, t17747) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2275::<F>(t1121, t1774, t17730, t3626, t3584, t471, t5351, t3720, t13142, t17708);
    (t17731, t17732, t17736, t17737, t17738, t17739, t17742, t17743, t17744, t17747)
}
